/* The field behind the page.
 *
 * The reference site drives its background with Three.js and a ShaderMaterial
 * over simplex noise. This does the same thing in about a hundred lines of raw
 * WebGL, because one fullscreen triangle does not need a scene graph — and a
 * site for a security tool has no business shipping 600 KB of library to draw
 * a moving gradient.
 *
 * Simplex noise below is Ashima Arts / Stefan Gustavson's well-known GLSL
 * implementation, MIT licensed.
 */

const VERT = `#version 300 es
precision highp float;
out vec2 vUv;
void main() {
  // one oversized triangle covering the clip volume; no vertex buffer needed
  vec2 p = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
  vUv = p;
  gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}`;

const FRAG = `#version 300 es
precision highp float;

in vec2 vUv;
out vec4 fragColor;

uniform vec2  uRes;
uniform float uTime;
uniform vec2  uPointer;
uniform float uIntensity;
uniform vec3  uAccent;

vec3 mod289(vec3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
vec4 mod289(vec4 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
vec4 permute(vec4 x) { return mod289(((x * 34.0) + 10.0) * x); }
vec4 taylorInvSqrt(vec4 r) { return 1.79284291400159 - 0.85373472095314 * r; }

float snoise(vec3 v) {
  const vec2 C = vec2(1.0 / 6.0, 1.0 / 3.0);
  const vec4 D = vec4(0.0, 0.5, 1.0, 2.0);

  vec3 i  = floor(v + dot(v, C.yyy));
  vec3 x0 = v - i + dot(i, C.xxx);

  vec3 g = step(x0.yzx, x0.xyz);
  vec3 l = 1.0 - g;
  vec3 i1 = min(g.xyz, l.zxy);
  vec3 i2 = max(g.xyz, l.zxy);

  vec3 x1 = x0 - i1 + C.xxx;
  vec3 x2 = x0 - i2 + C.yyy;
  vec3 x3 = x0 - D.yyy;

  i = mod289(i);
  vec4 p = permute(permute(permute(
             i.z + vec4(0.0, i1.z, i2.z, 1.0))
           + i.y + vec4(0.0, i1.y, i2.y, 1.0))
           + i.x + vec4(0.0, i1.x, i2.x, 1.0));

  float n_ = 0.142857142857;
  vec3 ns = n_ * D.wyz - D.xzx;

  vec4 j = p - 49.0 * floor(p * ns.z * ns.z);

  vec4 x_ = floor(j * ns.z);
  vec4 y_ = floor(j - 7.0 * x_);

  vec4 x = x_ * ns.x + ns.yyyy;
  vec4 y = y_ * ns.x + ns.yyyy;
  vec4 h = 1.0 - abs(x) - abs(y);

  vec4 b0 = vec4(x.xy, y.xy);
  vec4 b1 = vec4(x.zw, y.zw);

  vec4 s0 = floor(b0) * 2.0 + 1.0;
  vec4 s1 = floor(b1) * 2.0 + 1.0;
  vec4 sh = -step(h, vec4(0.0));

  vec4 a0 = b0.xzyw + s0.xzyw * sh.xxyy;
  vec4 a1 = b1.xzyw + s1.xzyw * sh.zzww;

  vec3 p0 = vec3(a0.xy, h.x);
  vec3 p1 = vec3(a0.zw, h.y);
  vec3 p2 = vec3(a1.xy, h.z);
  vec3 p3 = vec3(a1.zw, h.w);

  vec4 norm = taylorInvSqrt(vec4(dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)));
  p0 *= norm.x; p1 *= norm.y; p2 *= norm.z; p3 *= norm.w;

  vec4 m = max(0.5 - vec4(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)), 0.0);
  m = m * m;
  return 105.0 * dot(m * m, vec4(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)));
}

float fbm(vec3 p) {
  float sum = 0.0, amp = 0.5;
  for (int i = 0; i < 5; i++) {
    sum += amp * snoise(p);
    p *= 2.02;
    amp *= 0.5;
  }
  return sum;
}

void main() {
  vec2 uv = vUv;
  vec2 p = (uv - 0.5) * vec2(uRes.x / uRes.y, 1.0);

  float t = uTime * 0.045;

  // Warp the domain twice before sampling. One pass gives clouds; two gives
  // the drawn-out filaments that read as something being pulled into focus.
  vec3 q = vec3(p * 1.35, t);
  vec2 warp1 = vec2(fbm(q), fbm(q + vec3(5.2, 1.3, 0.0)));
  vec3 r = vec3(p * 1.35 + warp1 * 0.85, t * 1.2 + 2.3);
  vec2 warp2 = vec2(fbm(r), fbm(r + vec3(1.7, 9.2, 0.0)));

  float f = fbm(vec3(p * 1.6 + warp2 * 1.1, t * 0.8));
  f = f * 0.5 + 0.5;

  // Ridges: the thin bright lines where the field folds over on itself. The
  // exponent is what keeps them filaments rather than clouds — this is a
  // backdrop for reading against, so almost all of the frame stays black.
  float ridge = 1.0 - abs(f * 2.0 - 1.0);
  ridge = pow(clamp(ridge, 0.0, 1.0), 16.0);

  // The pointer lifts the field slightly, so the page answers the cursor
  // without anything appearing to chase it.
  float near = 1.0 - smoothstep(0.0, 0.8, distance(p, uPointer));

  vec3 base = vec3(0.0157, 0.0157, 0.0157);
  vec3 col = base;
  col += uAccent * ridge * (0.16 + near * 0.20) * uIntensity;
  col += uAccent * 0.014 * smoothstep(0.55, 1.0, f) * uIntensity;
  col += vec3(0.010) * pow(clamp(f, 0.0, 1.0), 4.0) * uIntensity;

  // Vignette, weighted to the left, because that is the side the type sits on
  // for most of the sections and it needs the quietest ground.
  vec2 vp = uv - vec2(0.34, 0.5);
  float vig = 1.0 - smoothstep(0.30, 1.05, length(vp * vec2(1.0, 1.15)) * 1.5);
  col *= 0.18 + 0.82 * vig;

  // a little ordered dither: banding is very visible on a near-black field
  float d = fract(sin(dot(gl_FragCoord.xy, vec2(12.9898, 78.233))) * 43758.5453);
  col += (d - 0.5) * 0.006;

  fragColor = vec4(col, 1.0);
}`;

function compile(gl, type, src) {
  const sh = gl.createShader(type);
  gl.shaderSource(sh, src);
  gl.compileShader(sh);
  if (!gl.getShaderParameter(sh, gl.COMPILE_STATUS)) {
    console.warn('shader:', gl.getShaderInfoLog(sh));
    gl.deleteShader(sh);
    return null;
  }
  return sh;
}

/**
 * Start the field on a container element. Returns a handle with `setIntensity`
 * and `destroy`, or null when WebGL2 is unavailable — the caller then leaves
 * the CSS fallback in place.
 */
export function startField(container, opts = {}) {
  const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  const canvas = document.createElement('canvas');
  const gl = canvas.getContext('webgl2', {
    antialias: false,
    alpha: false,
    powerPreference: 'low-power',
    preserveDrawingBuffer: false,
  });
  if (!gl) return null;

  const vs = compile(gl, gl.VERTEX_SHADER, VERT);
  const fs = compile(gl, gl.FRAGMENT_SHADER, FRAG);
  if (!vs || !fs) return null;

  const prog = gl.createProgram();
  gl.attachShader(prog, vs);
  gl.attachShader(prog, fs);
  gl.linkProgram(prog);
  if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
    console.warn('link:', gl.getProgramInfoLog(prog));
    return null;
  }
  gl.useProgram(prog);

  const u = {
    res: gl.getUniformLocation(prog, 'uRes'),
    time: gl.getUniformLocation(prog, 'uTime'),
    pointer: gl.getUniformLocation(prog, 'uPointer'),
    intensity: gl.getUniformLocation(prog, 'uIntensity'),
    accent: gl.getUniformLocation(prog, 'uAccent'),
  };
  gl.uniform3fv(u.accent, opts.accent || [0.29, 0.64, 0.79]);

  container.appendChild(canvas);

  // Noise is expensive per pixel, so render below device resolution and let the
  // browser scale it up. On a field this soft nobody can tell.
  const scale = Math.min(window.devicePixelRatio || 1, 1.5) * 0.7;
  let w = 0, h = 0;
  const resize = () => {
    const rect = container.getBoundingClientRect();
    w = Math.max(1, Math.floor(rect.width * scale));
    h = Math.max(1, Math.floor(rect.height * scale));
    canvas.width = w;
    canvas.height = h;
    canvas.style.width = '100%';
    canvas.style.height = '100%';
    gl.viewport(0, 0, w, h);
    gl.uniform2f(u.res, w, h);
  };
  resize();
  const ro = new ResizeObserver(resize);
  ro.observe(container);

  const pointer = { x: 0, y: 0, tx: 0, ty: 0 };
  const onMove = (e) => {
    const rect = container.getBoundingClientRect();
    if (!rect.width || !rect.height) return;
    pointer.tx = ((e.clientX - rect.left) / rect.width - 0.5) * (rect.width / rect.height);
    pointer.ty = -((e.clientY - rect.top) / rect.height - 0.5);
  };
  window.addEventListener('pointermove', onMove, { passive: true });

  let intensity = 1, target = 1;
  let raf = 0, running = true;
  const t0 = performance.now();
  let last = 0;

  const frame = (now) => {
    if (!running) return;
    raf = requestAnimationFrame(frame);
    // hold roughly 30fps: this is a backdrop, not a game
    if (now - last < 32) return;
    last = now;

    pointer.x += (pointer.tx - pointer.x) * 0.045;
    pointer.y += (pointer.ty - pointer.y) * 0.045;
    intensity += (target - intensity) * 0.05;

    gl.uniform1f(u.time, reduced ? 12.0 : (now - t0) / 1000);
    gl.uniform2f(u.pointer, pointer.x, pointer.y);
    gl.uniform1f(u.intensity, intensity);
    gl.drawArrays(gl.TRIANGLES, 0, 3);

    // A still page should not keep a GPU busy. With reduced motion we draw one
    // frame and stop until something changes.
    if (reduced && Math.abs(target - intensity) < 0.01) { running = false; }
  };
  raf = requestAnimationFrame(frame);

  const onVisibility = () => {
    if (document.hidden) {
      running = false;
      cancelAnimationFrame(raf);
    } else if (!running) {
      running = true;
      raf = requestAnimationFrame(frame);
    }
  };
  document.addEventListener('visibilitychange', onVisibility);

  return {
    setIntensity(v) {
      target = v;
      if (!running && !document.hidden) { running = true; raf = requestAnimationFrame(frame); }
    },
    destroy() {
      running = false;
      cancelAnimationFrame(raf);
      ro.disconnect();
      window.removeEventListener('pointermove', onMove);
      document.removeEventListener('visibilitychange', onVisibility);
      canvas.remove();
    },
  };
}
