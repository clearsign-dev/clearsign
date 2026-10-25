import { Ionicons } from '@expo/vector-icons';
import { StyleSheet, Text, View } from 'react-native';

import type { ReviewResponse, ReviewSeverity } from '../core/review';
import { type Colors, useColors } from '../theme';

const TITLES: Record<ReviewSeverity, string> = {
  clear: 'No blind or critical finding',
  warning: 'Review the warnings',
  blind: 'ClearSign cannot fully interpret this',
  critical: 'Do not approve this transaction',
  refused: 'Review refused',
};

function severityColors(colors: Colors, severity: ReviewSeverity) {
  if (severity === 'critical' || severity === 'refused') {
    return { strong: colors.critical, muted: colors.criticalMuted };
  }
  if (severity === 'blind' || severity === 'warning') {
    return { strong: colors.warning, muted: colors.warningMuted };
  }
  return { strong: colors.clear, muted: colors.clearMuted };
}

export function ReviewResult({ result }: { result: ReviewResponse }) {
  const colors = useColors();
  const severity = severityColors(colors, result.severity);
  return (
    <View style={styles.stack} testID="review-result">
      <View style={[styles.verdict, { backgroundColor: severity.muted, borderColor: severity.strong }]}>
        <Ionicons
          name={result.severity === 'clear' ? 'checkmark-circle' : 'warning'}
          size={25}
          color={severity.strong}
        />
        <View style={styles.grow}>
          <Text style={[styles.eyebrow, { color: severity.strong }]}>{result.severity.toUpperCase()}</Text>
          <Text style={[styles.verdictTitle, { color: colors.ink }]}>{TITLES[result.severity]}</Text>
          {result.message ? <Text style={[styles.message, { color: colors.inkSoft }]}>{result.message}</Text> : null}
        </View>
      </View>

      {result.hash ? (
        <View style={[styles.section, { backgroundColor: colors.surface, borderColor: colors.line }]}>
          <View style={styles.sectionHeading}>
            <Text style={[styles.label, { color: colors.inkFaint }]}>SIGNING HASH</Text>
            <View style={styles.confirmed}>
              <Ionicons
                name={result.hashConfirmed ? 'checkmark-circle' : 'help-circle-outline'}
                size={15}
                color={result.hashConfirmed ? colors.clear : colors.warning}
              />
              <Text style={[styles.confirmedText, { color: colors.inkSoft }]}>
                {result.hashConfirmed ? 'Matches record' : 'Not supplied in record'}
              </Text>
            </View>
          </View>
          <Text selectable style={[styles.hash, { color: colors.ink }]}>{result.hash}</Text>
          <View style={[styles.meta, { borderTopColor: colors.line }]}>
            <Text style={[styles.metaText, { color: colors.inkSoft }]}>Safe {result.domain}</Text>
            <Text style={[styles.metaText, { color: colors.inkSoft }]}>
              {result.chainBound ? 'Chain-bound domain' : 'Domain is not chain-bound'}
            </Text>
          </View>
        </View>
      ) : null}

      {(result.findings ?? []).length > 0 ? (
        <View style={styles.group}>
          <Text style={[styles.groupTitle, { color: colors.inkSoft }]}>Findings</Text>
          {(result.findings ?? []).map((finding, index) => {
            const findingSeverity = severityColors(
              colors,
              finding.severity.toLowerCase() as ReviewSeverity,
            );
            return (
              <View
                key={`${finding.number}-${finding.code}-${index}`}
                style={[styles.finding, { backgroundColor: colors.surface, borderColor: colors.line }]}
              >
                <View style={[styles.findingBar, { backgroundColor: findingSeverity.strong }]} />
                <View style={styles.grow}>
                  <Text style={[styles.findingCode, { color: findingSeverity.strong }]}>#{finding.number} {finding.code}</Text>
                  <Text style={[styles.findingMessage, { color: colors.ink }]}>{finding.message}</Text>
                </View>
              </View>
            );
          })}
        </View>
      ) : null}

      {(result.sections ?? []).map((section) => (
        <View key={section.title} style={[styles.section, { backgroundColor: colors.surface, borderColor: colors.line }]}>
          <Text style={[styles.groupTitle, { color: colors.inkSoft }]}>{section.title}</Text>
          {section.fields.map(([name, value]) => (
            <View key={`${section.title}-${name}`} style={[styles.field, { borderTopColor: colors.line }]}>
              <Text style={[styles.fieldName, { color: colors.inkFaint }]}>{name}</Text>
              <Text selectable style={[styles.fieldValue, { color: colors.ink }]}>{value}</Text>
            </View>
          ))}
        </View>
      ))}
    </View>
  );
}

const styles = StyleSheet.create({
  stack: { gap: 12 },
  grow: { flex: 1, minWidth: 0 },
  verdict: {
    borderLeftWidth: 5,
    borderRadius: 8,
    borderWidth: 1,
    flexDirection: 'row',
    gap: 12,
    padding: 16,
  },
  eyebrow: { fontSize: 10, fontWeight: '800', letterSpacing: 0 },
  verdictTitle: { fontSize: 19, fontWeight: '700', letterSpacing: 0, marginTop: 2 },
  message: { fontSize: 13.5, lineHeight: 20, marginTop: 7 },
  section: { borderRadius: 8, borderWidth: 1, overflow: 'hidden', padding: 15 },
  sectionHeading: { alignItems: 'center', flexDirection: 'row', justifyContent: 'space-between' },
  label: { fontSize: 10, fontWeight: '800', letterSpacing: 0 },
  confirmed: { alignItems: 'center', flexDirection: 'row', gap: 5 },
  confirmedText: { fontSize: 11.5 },
  hash: { fontFamily: 'monospace', fontSize: 15, fontWeight: '600', lineHeight: 23, marginTop: 10 },
  meta: { borderTopWidth: 1, flexDirection: 'row', flexWrap: 'wrap', gap: 8, justifyContent: 'space-between', marginTop: 12, paddingTop: 10 },
  metaText: { fontSize: 11.5 },
  group: { gap: 8 },
  groupTitle: { fontSize: 11, fontWeight: '800', letterSpacing: 0, textTransform: 'uppercase' },
  finding: { borderRadius: 7, borderWidth: 1, flexDirection: 'row', overflow: 'hidden' },
  findingBar: { width: 4 },
  findingCode: { fontFamily: 'monospace', fontSize: 11, fontWeight: '700', letterSpacing: 0, paddingHorizontal: 12, paddingTop: 11 },
  findingMessage: { fontSize: 13.5, lineHeight: 20, paddingBottom: 12, paddingHorizontal: 12, paddingTop: 4 },
  field: { borderTopWidth: StyleSheet.hairlineWidth, gap: 4, marginTop: 10, paddingTop: 10 },
  fieldName: { fontSize: 11.5 },
  fieldValue: { fontFamily: 'monospace', fontSize: 12.5, lineHeight: 19 },
});
