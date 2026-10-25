import { useColorScheme } from 'react-native';

const light = {
  paper: '#EEF1F4',
  surface: '#FFFFFF',
  surfaceMuted: '#F4F6F7',
  ink: '#121518',
  inkSoft: '#536068',
  inkFaint: '#77838B',
  line: '#D5DCE0',
  lineStrong: '#AEB8BF',
  accent: '#167590',
  accentMuted: '#E3F1F4',
  critical: '#B6322C',
  criticalMuted: '#FFF0ED',
  warning: '#8A5A00',
  warningMuted: '#FFF5DC',
  clear: '#1D7248',
  clearMuted: '#EAF6EF',
  white: '#FFFFFF',
};

const dark: typeof light = {
  paper: '#111416',
  surface: '#191D20',
  surfaceMuted: '#121619',
  ink: '#F2F4F5',
  inkSoft: '#B0B9BF',
  inkFaint: '#7E8A92',
  line: '#2C3338',
  lineStrong: '#485159',
  accent: '#73C3D7',
  accentMuted: '#14282E',
  critical: '#F2938C',
  criticalMuted: '#2B1512',
  warning: '#E0B562',
  warningMuted: '#2B2212',
  clear: '#78C997',
  clearMuted: '#12251A',
  white: '#FFFFFF',
};

export type Colors = typeof light;

export function useColors(): Colors {
  return useColorScheme() === 'dark' ? dark : light;
}
