import { Ionicons } from '@expo/vector-icons';
import * as Clipboard from 'expo-clipboard';
import * as DocumentPicker from 'expo-document-picker';
import { File } from 'expo-file-system';
import { useRef, useState } from 'react';
import {
  ActivityIndicator,
  KeyboardAvoidingView,
  Platform,
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  TextInput,
  type ScrollViewInstance,
  useWindowDimensions,
  View,
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import { BrandHeader } from '../components/BrandHeader';
import { ReviewResult } from '../components/ReviewResult';
import { engineAvailable, reviewSafeJson, type ReviewResponse } from '../core/review';
import { useColors } from '../theme';

const bybitFixture = require('../../assets/fixtures/bybit-safe-tx.json');
const MAX_RECORD_BYTES = 1024 * 1024;

export default function ReviewScreen() {
  const colors = useColors();
  const { width } = useWindowDimensions();
  const wide = width >= 860;
  const scroll = useRef<ScrollViewInstance>(null);
  const requestGeneration = useRef(0);
  const [jsonText, setJsonText] = useState('');
  const [chainId, setChainId] = useState('1');
  const [version, setVersion] = useState(0);
  const [result, setResult] = useState<ReviewResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [working, setWorking] = useState(false);

  const invalidateReview = () => {
    requestGeneration.current += 1;
    setWorking(false);
    setResult(null);
    setError(null);
  };

  const replaceInput = (text: string) => {
    invalidateReview();
    const byteLength = utf8ByteLength(text, MAX_RECORD_BYTES);
    if (byteLength > MAX_RECORD_BYTES) {
      setError(`The transaction record exceeds the ${MAX_RECORD_BYTES.toLocaleString()}-byte review limit.`);
      return;
    }
    setJsonText(text);
  };

  const paste = async () => {
    const generation = requestGeneration.current;
    const text = await Clipboard.getStringAsync();
    if (requestGeneration.current === generation) replaceInput(text);
  };

  const pickFile = async () => {
    const generation = requestGeneration.current;
    const picked = await DocumentPicker.getDocumentAsync({
      copyToCacheDirectory: true,
      type: ['application/json', 'text/json', 'text/plain'],
    });
    if (picked.canceled) return;
    if (requestGeneration.current !== generation) return;
    const asset = picked.assets[0];
    if (!asset) return;
    const file = new File(asset.uri);
    const size = file.size ?? asset.size;
    if (typeof size !== 'number') {
      invalidateReview();
      setError('ClearSign could not verify the selected file size, so it was not opened.');
      return;
    }
    if (size > MAX_RECORD_BYTES) {
      invalidateReview();
      setError(`That file is ${size.toLocaleString()} bytes. The review limit is ${MAX_RECORD_BYTES.toLocaleString()}.`);
      return;
    }
    const text = await file.text();
    if (requestGeneration.current === generation) replaceInput(text);
  };

  const review = async () => {
    const request = requestGeneration.current + 1;
    requestGeneration.current = request;
    setWorking(true);
    setError(null);
    setResult(null);
    try {
      const response = await reviewSafeJson(jsonText, chainId.trim(), version);
      if (requestGeneration.current !== request) return;
      setResult(response);
      if (!wide) requestAnimationFrame(() => scroll.current?.scrollToEnd({ animated: true }));
    } catch (caught) {
      if (requestGeneration.current !== request) return;
      setError(caught instanceof Error ? caught.message : 'The review could not be completed.');
    } finally {
      if (requestGeneration.current === request) setWorking(false);
    }
  };

  return (
    <SafeAreaView style={[styles.safe, { backgroundColor: colors.paper }]} edges={['top', 'left', 'right']}>
      <KeyboardAvoidingView style={styles.safe} behavior={Platform.OS === 'ios' ? 'padding' : undefined}>
        <ScrollView ref={scroll} contentContainerStyle={styles.scroll} keyboardShouldPersistTaps="handled">
          <View style={styles.maxWidth}>
            <BrandHeader />
            <View style={[styles.localStrip, { backgroundColor: colors.surface, borderColor: colors.line }]}>
              <View style={[styles.localDot, { backgroundColor: engineAvailable ? colors.clear : colors.warning }]} />
              <Text style={[styles.localText, { color: colors.inkSoft }]}>
                {engineAvailable
                  ? 'Rust engine loaded. Reviews stay on this device.'
                  : 'Native engine unavailable in this preview build.'}
              </Text>
            </View>

            <View style={[styles.workspace, wide && styles.workspaceWide]}>
              <View style={[styles.inputPanel, wide && styles.inputPanelWide, { backgroundColor: colors.surface, borderColor: colors.line }]}>
                <View style={styles.panelHeading}>
                  <View>
                    <Text style={[styles.kicker, { color: colors.inkFaint }]}>SAFE TRANSACTION</Text>
                    <Text style={[styles.heading, { color: colors.ink }]}>Review the signed fields</Text>
                  </View>
                  <Pressable
                    accessibilityLabel="Load attack example"
                    onPress={() => replaceInput(JSON.stringify(bybitFixture, null, 2))}
                    style={styles.exampleButton}
                  >
                    <Ionicons name="flask-outline" size={16} color={colors.accent} />
                    <Text style={[styles.exampleText, { color: colors.accent }]}>Example</Text>
                  </Pressable>
                </View>

                <Text style={[styles.helper, { color: colors.inkSoft }]}>Paste the JSON you received from Safe, or import the saved record.</Text>
                <View style={styles.toolRow}>
                  <Pressable onPress={paste} style={[styles.toolButton, { borderColor: colors.lineStrong }]}>
                    <Ionicons name="clipboard-outline" size={17} color={colors.ink} />
                    <Text style={[styles.toolText, { color: colors.ink }]}>Paste</Text>
                  </Pressable>
                  <Pressable onPress={pickFile} style={[styles.toolButton, { borderColor: colors.lineStrong }]}>
                    <Ionicons name="document-outline" size={17} color={colors.ink} />
                    <Text style={[styles.toolText, { color: colors.ink }]}>Import file</Text>
                  </Pressable>
                </View>
                <TextInput
                  accessibilityLabel="Safe transaction JSON"
                  autoCapitalize="none"
                  autoCorrect={false}
                  multiline
                  onChangeText={replaceInput}
                  placeholder="Paste Safe transaction JSON"
                  placeholderTextColor={colors.inkFaint}
                  spellCheck={false}
                  style={[styles.editor, { backgroundColor: colors.surfaceMuted, borderColor: colors.line, color: colors.ink }]}
                  textAlignVertical="top"
                  value={jsonText}
                />

                <View style={styles.fieldsRow}>
                  <View style={styles.chainField}>
                    <Text style={[styles.fieldLabel, { color: colors.inkSoft }]}>Chain ID</Text>
                    <TextInput
                      accessibilityLabel="Chain ID"
                      keyboardType="number-pad"
                      onChangeText={(value) => { invalidateReview(); setChainId(value); }}
                      style={[styles.smallInput, { backgroundColor: colors.surfaceMuted, borderColor: colors.line, color: colors.ink }]}
                      value={chainId}
                    />
                  </View>
                  <View style={styles.domainField}>
                    <Text style={[styles.fieldLabel, { color: colors.inkSoft }]}>Safe domain</Text>
                    <View style={[styles.segmented, { backgroundColor: colors.surfaceMuted, borderColor: colors.line }]}>
                      {[
                        ['Auto', 0],
                        ['v1.1', 1],
                        ['v1.3+', 3],
                      ].map(([label, value]) => (
                        <Pressable
                          key={String(value)}
                          onPress={() => { invalidateReview(); setVersion(Number(value)); }}
                          style={[styles.segment, version === value && { backgroundColor: colors.surface }]}
                        >
                          <Text style={[styles.segmentText, { color: version === value ? colors.ink : colors.inkFaint }]}>{label}</Text>
                        </Pressable>
                      ))}
                    </View>
                  </View>
                </View>

                {error ? (
                  <View style={[styles.error, { backgroundColor: colors.criticalMuted, borderColor: colors.critical }]}>
                    <Ionicons name="alert-circle-outline" size={18} color={colors.critical} />
                    <Text style={[styles.errorText, { color: colors.ink }]}>{error}</Text>
                  </View>
                ) : null}

                <Pressable
                  accessibilityRole="button"
                  disabled={working || jsonText.trim().length === 0}
                  onPress={review}
                  style={({ pressed }) => [
                    styles.reviewButton,
                    { backgroundColor: colors.accent, opacity: working || jsonText.trim().length === 0 ? 0.45 : pressed ? 0.82 : 1 },
                  ]}
                >
                  {working ? <ActivityIndicator color={colors.white} /> : <Ionicons name="scan" size={19} color={colors.white} />}
                  <Text style={[styles.reviewText, { color: colors.white }]}>{working ? 'Reviewing locally' : 'Review transaction'}</Text>
                </Pressable>
                <Text style={[styles.boundary, { color: colors.inkFaint }]}>Only signed fields are read. Service-provided decoded descriptions are ignored.</Text>
              </View>

              <View style={[styles.resultColumn, wide && styles.resultColumnWide]}>
                {result ? (
                  <ReviewResult result={result} />
                ) : (
                  <View style={[styles.empty, { backgroundColor: colors.surface, borderColor: colors.line }]}>
                    <Ionicons name="finger-print-outline" size={34} color={colors.accent} />
                    <Text style={[styles.emptyTitle, { color: colors.ink }]}>The exact review appears here</Text>
                    <Text style={[styles.emptyText, { color: colors.inkSoft }]}>You will see the locally recomputed hash, signed fields and every finding before you approve elsewhere.</Text>
                  </View>
                )}
              </View>
            </View>
          </View>
        </ScrollView>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}

function utf8ByteLength(value: string, stopAfter: number): number {
  let bytes = 0;
  for (const character of value) {
    const codePoint = character.codePointAt(0) ?? 0;
    bytes += codePoint <= 0x7f ? 1 : codePoint <= 0x7ff ? 2 : codePoint <= 0xffff ? 3 : 4;
    if (bytes > stopAfter) return bytes;
  }
  return bytes;
}

const styles = StyleSheet.create({
  safe: { flex: 1 },
  scroll: { padding: 16, paddingBottom: 36 },
  maxWidth: { alignSelf: 'center', maxWidth: 1180, width: '100%' },
  localStrip: { alignItems: 'center', borderRadius: 7, borderWidth: 1, flexDirection: 'row', gap: 8, marginTop: 12, paddingHorizontal: 11, paddingVertical: 9 },
  localDot: { borderRadius: 4, height: 7, width: 7 },
  localText: { flex: 1, fontSize: 12.5, lineHeight: 18 },
  workspace: { gap: 14, marginTop: 14 },
  workspaceWide: { alignItems: 'flex-start', flexDirection: 'row' },
  inputPanel: { borderRadius: 8, borderWidth: 1, padding: 16 },
  inputPanelWide: { flexBasis: 390, flexGrow: 0, flexShrink: 0 },
  resultColumn: { minWidth: 0 },
  resultColumnWide: { flex: 1 },
  panelHeading: { alignItems: 'center', flexDirection: 'row', justifyContent: 'space-between' },
  kicker: { fontSize: 10, fontWeight: '800', letterSpacing: 0 },
  heading: { fontSize: 18, fontWeight: '700', letterSpacing: 0, marginTop: 2 },
  exampleButton: { alignItems: 'center', flexDirection: 'row', gap: 5, minHeight: 36, padding: 6 },
  exampleText: { fontSize: 12.5, fontWeight: '600' },
  helper: { fontSize: 12.5, lineHeight: 18, marginTop: 14 },
  toolRow: { flexDirection: 'row', gap: 8, marginTop: 10 },
  toolButton: { alignItems: 'center', borderRadius: 6, borderWidth: 1, flexDirection: 'row', gap: 7, justifyContent: 'center', minHeight: 40, paddingHorizontal: 13 },
  toolText: { fontSize: 13, fontWeight: '600' },
  editor: { borderRadius: 7, borderWidth: 1, fontFamily: 'monospace', fontSize: 12.5, lineHeight: 18, marginTop: 10, minHeight: 235, padding: 12 },
  fieldsRow: { flexDirection: 'row', gap: 10, marginTop: 12 },
  chainField: { flexBasis: 105 },
  domainField: { flex: 1 },
  fieldLabel: { fontSize: 11.5, marginBottom: 6 },
  smallInput: { borderRadius: 6, borderWidth: 1, fontSize: 14, height: 42, paddingHorizontal: 11 },
  segmented: { borderRadius: 7, borderWidth: 1, flexDirection: 'row', height: 42, padding: 3 },
  segment: { alignItems: 'center', borderRadius: 5, flex: 1, justifyContent: 'center' },
  segmentText: { fontSize: 11.5, fontWeight: '700' },
  error: { alignItems: 'flex-start', borderRadius: 7, borderWidth: 1, flexDirection: 'row', gap: 8, marginTop: 12, padding: 11 },
  errorText: { flex: 1, fontSize: 12.5, lineHeight: 18 },
  reviewButton: { alignItems: 'center', borderRadius: 7, flexDirection: 'row', gap: 8, justifyContent: 'center', marginTop: 14, minHeight: 48 },
  reviewText: { fontSize: 14, fontWeight: '700' },
  boundary: { fontSize: 11.5, lineHeight: 17, marginTop: 10, textAlign: 'center' },
  empty: { alignItems: 'center', borderRadius: 8, borderWidth: 1, minHeight: 280, justifyContent: 'center', padding: 30 },
  emptyTitle: { fontSize: 18, fontWeight: '700', letterSpacing: 0, marginTop: 14, textAlign: 'center' },
  emptyText: { fontSize: 13.5, lineHeight: 20, marginTop: 7, maxWidth: 420, textAlign: 'center' },
});
