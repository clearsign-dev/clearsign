import { Ionicons } from '@expo/vector-icons';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import { BrandHeader } from '../components/BrandHeader';
import { useColors } from '../theme';

const checks = [
  ['Recomputes the Safe signing hash', 'The hash comes from the signed fields, not the service description.'],
  ['Names dangerous operation types', 'DELEGATECALL, Safe administration and supported nested calls are surfaced directly.'],
  ['Refuses unsupported certainty', 'Unknown or only partly understood content is BLIND or refused, never guessed.'],
];

const boundaries = [
  'This app currently accepts Safe transaction JSON. It is not a general wallet and holds no keys.',
  'A decoded selector does not prove what deployed contract code will do.',
  'The dedicated seL4 signing platform remains an emulated research system, separate from this phone app.',
];

export default function ScopeScreen() {
  const colors = useColors();
  return (
    <SafeAreaView style={[styles.safe, { backgroundColor: colors.paper }]} edges={['top', 'left', 'right']}>
      <ScrollView contentContainerStyle={styles.scroll}>
        <View style={styles.maxWidth}>
          <BrandHeader />
          <Text style={[styles.title, { color: colors.ink }]}>What this build can establish</Text>
          <Text style={[styles.intro, { color: colors.inkSoft }]}>ClearSign is a local verification engine beside your wallet. It reviews; it does not connect accounts, hold keys or sign.</Text>

          <View style={styles.list}>
            {checks.map(([title, detail]) => (
              <View key={title} style={[styles.row, { backgroundColor: colors.surface, borderColor: colors.line }]}>
                <Ionicons name="checkmark-circle-outline" size={22} color={colors.clear} />
                <View style={styles.copy}>
                  <Text style={[styles.rowTitle, { color: colors.ink }]}>{title}</Text>
                  <Text style={[styles.rowDetail, { color: colors.inkSoft }]}>{detail}</Text>
                </View>
              </View>
            ))}
          </View>

          <Text style={[styles.sectionTitle, { color: colors.ink }]}>Boundaries</Text>
          <View style={[styles.boundaryPanel, { backgroundColor: colors.warningMuted, borderColor: colors.warning }]}>
            {boundaries.map((boundary) => (
              <View key={boundary} style={styles.boundaryRow}>
                <View style={[styles.bullet, { backgroundColor: colors.warning }]} />
                <Text style={[styles.boundaryText, { color: colors.ink }]}>{boundary}</Text>
              </View>
            ))}
          </View>

          <Text style={[styles.version, { color: colors.inkFaint }]}>Mobile evaluation build 0.1.6 · Review API v1</Text>
        </View>
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1 },
  scroll: { padding: 16, paddingBottom: 36 },
  maxWidth: { alignSelf: 'center', maxWidth: 760, width: '100%' },
  title: { fontSize: 25, fontWeight: '800', letterSpacing: 0, marginTop: 30 },
  intro: { fontSize: 14.5, lineHeight: 22, marginTop: 8, maxWidth: 620 },
  list: { gap: 9, marginTop: 22 },
  row: { alignItems: 'flex-start', borderRadius: 8, borderWidth: 1, flexDirection: 'row', gap: 12, padding: 15 },
  copy: { flex: 1 },
  rowTitle: { fontSize: 15, fontWeight: '700', letterSpacing: 0 },
  rowDetail: { fontSize: 13, lineHeight: 19, marginTop: 4 },
  sectionTitle: { fontSize: 18, fontWeight: '700', letterSpacing: 0, marginTop: 28 },
  boundaryPanel: { borderLeftWidth: 4, borderRadius: 8, borderWidth: 1, gap: 12, marginTop: 10, padding: 15 },
  boundaryRow: { alignItems: 'flex-start', flexDirection: 'row', gap: 9 },
  bullet: { borderRadius: 3, height: 6, marginTop: 7, width: 6 },
  boundaryText: { flex: 1, fontSize: 13.5, lineHeight: 20 },
  version: { fontSize: 11.5, marginTop: 24, textAlign: 'center' },
});
