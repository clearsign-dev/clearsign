import { Ionicons } from '@expo/vector-icons';
import { Image, StyleSheet, Text, View } from 'react-native';

import { useColors } from '../theme';

export function BrandHeader() {
  const colors = useColors();
  return (
    <View style={styles.row}>
      <Image source={require('../../assets/icon.png')} style={styles.mark} />
      <View style={styles.copy}>
        <Text style={[styles.name, { color: colors.ink }]}>ClearSign</Text>
        <Text style={[styles.tagline, { color: colors.inkSoft }]}>Independent transaction verification</Text>
      </View>
      <Ionicons name="shield-checkmark-outline" size={22} color={colors.accent} />
    </View>
  );
}

const styles = StyleSheet.create({
  row: {
    alignItems: 'center',
    flexDirection: 'row',
    gap: 11,
    minHeight: 48,
  },
  mark: {
    borderRadius: 8,
    height: 36,
    width: 36,
  },
  copy: { flex: 1 },
  name: { fontSize: 18, fontWeight: '700', letterSpacing: 0 },
  tagline: { fontSize: 11.5, letterSpacing: 0, marginTop: 1 },
});
