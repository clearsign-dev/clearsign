/* ClearSign — page behaviour.
 *
 * The page is a stage, not a scroll. Every section occupies the viewport and
 * only one is active; the wheel, arrow keys, swipes, the tick marks down the
 * side and the menu all do the same thing, which is change which one that is.
 */

import { startField } from './canvas.js';

const $  = (sel, root = document) => root.querySelector(sel);
const $$ = (sel, root = document) => Array.from(root.querySelectorAll(sel));

/* ------------------------------------------------------ letter splitting -- */

/* Wrap each character in its own element so the CSS can stagger them. Words
   stay whole so nothing breaks mid-word, and the original text is kept on the
   element for anything reading the page rather than looking at it. */
function splitText(el) {
  const text = el.textContent;
  el.setAttribute('aria-label', text.trim());
  const frag = document.createDocumentFragment();
  for (const word of text.split(/(\s+)/)) {
    if (!word) continue;
    if (/^\s+$/.test(word)) { frag.appendChild(document.createTextNode(' ')); continue; }
    const w = document.createElement('span');
    w.className = 'word';
    w.setAttribute('aria-hidden', 'true');
    for (const ch of Array.from(word)) {
      const c = document.createElement('span');
      c.className = 'letter';
      c.textContent = ch;
      w.appendChild(c);
    }
    frag.appendChild(w);
  }
  el.textContent = '';
  el.appendChild(frag);
}

/* ---------------------------------------------------------------- stage -- */

class Stage {
  constructor() {
    this.sections = $$('.section');
    this.names = this.sections.map((s) => s.dataset.name || s.id);
    this.index = 0;
    this.locked = false;
    this.field = null;

    this.controls = $('#controls');
    this.footerNow = $('#footerNow');
    this.footerTotal = $('#footerTotal');

    this.buildControls();
    this.bindMenu();
    this.bindInput();
    this.bindAccordions();
    this.bindTabs();
    this.bindShip();
    this.bindForm();

    for (const el of $$('[data-split]')) splitText(el);

    if (this.footerTotal) {
      this.footerTotal.textContent = String(this.sections.length).padStart(2, '0');
    }

    const fromHash = this.names.indexOf((location.hash || '').replace('#', ''));
    this.go(fromHash >= 0 ? fromHash : 0, true);

    window.addEventListener('hashchange', () => {
      const i = this.names.indexOf((location.hash || '').replace('#', ''));
      if (i >= 0 && i !== this.index) this.go(i);
    });
  }

  buildControls() {
    if (!this.controls) return;
    this.sections.forEach((section, i) => {
      const b = document.createElement('button');
      b.type = 'button';
      const label = section.dataset.label || this.names[i];
      b.innerHTML = `<i>${label}</i><span class="srOnly">Go to ${label}</span>`;
      b.addEventListener('click', () => this.go(i));
      this.controls.appendChild(b);
    });
    this.controlButtons = $$('button', this.controls);
  }

  go(i, immediate = false) {
    if (i < 0 || i >= this.sections.length) return;
    if (this.locked && !immediate) return;
    this.index = i;

    this.sections.forEach((s, n) => {
      s.classList.toggle('active', n === i);
      if (n === i) s.scrollTop = 0;
    });
    (this.controlButtons || []).forEach((b, n) => b.classList.toggle('active', n === i));
    $$('#menuList button').forEach((b, n) => b.classList.toggle('current', n === i));

    if (this.footerNow) this.footerNow.textContent = String(i + 1).padStart(2, '0');

    const name = this.names[i];
    if (name && location.hash.replace('#', '') !== name) {
      history.replaceState(null, '', '#' + name);
    }
    document.title = `${this.sections[i].dataset.title || 'ClearSign'} — ClearSign`;

    // The field brightens on the opening section and settles for reading ones.
    if (this.field) this.field.setIntensity(i === 0 ? 1 : 0.62);

    // Restarting the per-letter animation means removing and re-adding the class.
    const titles = $$('[data-split]', this.sections[i]);
    for (const t of titles) {
      t.style.animation = 'none';
      void t.offsetWidth;
      t.style.animation = '';
    }

    if (!immediate) {
      this.locked = true;
      setTimeout(() => { this.locked = false; }, 620);
    }
  }

  next() { this.go(Math.min(this.index + 1, this.sections.length - 1)); }
  prev() { this.go(Math.max(this.index - 1, 0)); }

  /* True when the active section has more content than fits and has not yet
     been scrolled to the edge we are heading towards. */
  atEdge(direction) {
    const el = this.sections[this.index];
    const slack = el.scrollHeight - el.clientHeight;
    if (slack <= 4) return true;
    return direction > 0
      ? el.scrollTop >= slack - 4
      : el.scrollTop <= 4;
  }

  bindInput() {
    let wheelAt = 0;
    window.addEventListener('wheel', (e) => {
      if (this.menuOpen) return;
      // let anything that genuinely scrolls keep its wheel
      if (e.target instanceof Element && e.target.closest('[data-scrolls]')) return;
      if (Math.abs(e.deltaY) < 12) return;
      const dir = e.deltaY > 0 ? 1 : -1;
      if (!this.atEdge(dir)) return;
      const now = Date.now();
      if (now - wheelAt < 700) return;
      wheelAt = now;
      dir > 0 ? this.next() : this.prev();
    }, { passive: true });

    window.addEventListener('keydown', (e) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;
      switch (e.key) {
        case 'ArrowDown': case 'PageDown': case ' ': e.preventDefault(); this.next(); break;
        case 'ArrowUp': case 'PageUp': e.preventDefault(); this.prev(); break;
        case 'Home': e.preventDefault(); this.go(0); break;
        case 'End': e.preventDefault(); this.go(this.sections.length - 1); break;
        case 'Escape': if (this.menuOpen) this.setMenu(false); break;
      }
    });

    let sy = null;
    window.addEventListener('touchstart', (e) => { sy = e.touches[0].clientY; }, { passive: true });
    window.addEventListener('touchend', (e) => {
      if (sy === null || this.menuOpen) return;
      const dy = sy - e.changedTouches[0].clientY;
      sy = null;
      if (Math.abs(dy) < 55) return;
      const dir = dy > 0 ? 1 : -1;
      if (!this.atEdge(dir)) return;
      dir > 0 ? this.next() : this.prev();
    }, { passive: true });
  }

  bindMenu() {
    this.menuOpen = false;
    const button = $('#menuButton');
    const menu = $('#menu');
    const list = $('#menuList');
    if (!button || !menu || !list) return;

    this.sections.forEach((section, i) => {
      const li = document.createElement('li');
      const b = document.createElement('button');
      b.type = 'button';
      const label = section.dataset.label || this.names[i];
      b.innerHTML = `<span aria-hidden="true">${label}</span>${label}<span aria-hidden="true">${label}</span>`;
      b.addEventListener('click', () => { this.setMenu(false); this.go(i); });
      li.appendChild(b);
      list.querySelector('ul').appendChild(li);
    });

    button.addEventListener('click', () => this.setMenu(!this.menuOpen));
  }

  setMenu(open) {
    this.menuOpen = open;
    $('#menu').classList.toggle('active', open);
    $('#menuList').classList.toggle('active', open);
    $('#menuButton').classList.toggle('active', open);
    $('#menuButton').setAttribute('aria-expanded', String(open));
    $('#menu').setAttribute('aria-hidden', String(!open));
    if (this.field) this.field.setIntensity(open ? 1.25 : (this.index === 0 ? 1 : 0.62));
  }

  bindAccordions() {
    for (const h of $$('.readsList h3')) {
      h.setAttribute('role', 'button');
      h.setAttribute('tabindex', '0');
      h.setAttribute('aria-expanded', 'false');
      const open = () => {
        const body = h.nextElementSibling;
        const isOpen = h.classList.contains('active');
        // one open at a time, as the reference does
        for (const other of $$('.readsList h3')) {
          other.classList.remove('active');
          other.setAttribute('aria-expanded', 'false');
          other.nextElementSibling?.classList.remove('active');
        }
        if (!isOpen) {
          h.classList.add('active');
          h.setAttribute('aria-expanded', 'true');
          body?.classList.add('active');
        }
      };
      h.addEventListener('click', open);
      h.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); open(); }
      });
    }
    // start with the first one open, so the section is never a list of closed doors
    $('.readsList h3')?.click();
  }

  bindTabs() {
    const tabs = $$('#verifiedTabs button');
    const groups = $$('[data-group]');
    if (!tabs.length) return;
    const select = (key) => {
      tabs.forEach((t) => t.classList.toggle('active', t.dataset.tab === key));
      groups.forEach((g) => { g.hidden = g.dataset.group !== key; });
      // re-run the fade so switching a tab looks like something happened
      for (const item of $$(`[data-group="${key}"] .topItem`)) {
        item.style.animation = 'none';
        void item.offsetWidth;
        item.style.animation = '';
      }
    };
    tabs.forEach((t) => t.addEventListener('click', () => select(t.dataset.tab)));
    select(tabs[0].dataset.tab);
  }

  bindShip() {
    const tabs = $$('#shipTabs button');
    const cards = $$('.shipCard');
    if (!tabs.length) return;
    const select = (key) => {
      tabs.forEach((t) => {
        const on = t.dataset.ship === key;
        t.classList.toggle('active', on);
        t.setAttribute('aria-selected', String(on));
      });
      cards.forEach((c) => c.classList.toggle('active', c.dataset.ship === key));
    };
    tabs.forEach((t) => t.addEventListener('click', () => select(t.dataset.ship)));
    select(tabs[0].dataset.ship);
  }

  /* There is no server behind this page and no analytics on it, so the form
     composes a mail rather than posting anywhere. It says so, too. */
  bindForm() {
    const form = $('#contactForm');
    if (!form) return;
    form.addEventListener('submit', (e) => {
      e.preventDefault();
      const data = new FormData(form);
      const name = String(data.get('name') || '').trim();
      const from = String(data.get('email') || '').trim();
      const body = String(data.get('message') || '').trim();
      const subject = encodeURIComponent(`ClearSign — ${name || 'enquiry'}`);
      const lines = [body, '', '--', name, from].filter(Boolean).join('\n');
      const to = form.dataset.mailto || 'hello@clearsign.dev';
      window.location.href = `mailto:${to}?subject=${subject}&body=${encodeURIComponent(lines)}`;
    });
  }
}

/* ------------------------------------------------------------- start up -- */

function boot() {
  const holder = $('#canvas');
  const stage = new Stage();
  if (holder) {
    stage.field = startField(holder, { accent: [0.29, 0.64, 0.79] });
    if (!stage.field) holder.classList.add('fallback');
  }

  // the menu gets its own, dimmer field so the overlay is not a flat black wall
  const menuHolder = $('#menuField');
  if (menuHolder) startField(menuHolder, { accent: [0.24, 0.52, 0.66] });

  const pre = $('#preloader');
  const bar = $('#preloaderBar span');
  const pct = $('#preloaderPct');
  let p = 0;
  const tick = setInterval(() => {
    p = Math.min(p + Math.random() * 18, 92);
    if (bar) bar.style.width = p + '%';
    if (pct) pct.textContent = String(Math.floor(p)).padStart(3, '0');
  }, 120);

  const finish = () => {
    clearInterval(tick);
    if (bar) bar.style.width = '100%';
    if (pct) pct.textContent = '100';
    setTimeout(() => pre?.classList.add('done'), 260);
  };

  const fonts = document.fonts ? document.fonts.ready : Promise.resolve();
  Promise.race([fonts, new Promise((r) => setTimeout(r, 2500))]).then(() => {
    setTimeout(finish, 320);
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', boot);
} else {
  boot();
}
