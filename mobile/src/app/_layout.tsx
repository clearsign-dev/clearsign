import { Ionicons } from '@expo/vector-icons';
import { Tabs } from 'expo-router';
import { StatusBar } from 'expo-status-bar';

import { useColors } from '../theme';

export default function RootLayout() {
  const colors = useColors();
  return (
    <>
      <StatusBar style="auto" />
      <Tabs
        screenOptions={{
          headerShown: false,
          tabBarActiveTintColor: colors.accent,
          tabBarInactiveTintColor: colors.inkFaint,
          tabBarStyle: { backgroundColor: colors.surface, borderTopColor: colors.line },
          tabBarLabelStyle: { fontSize: 11, fontWeight: '600', letterSpacing: 0 },
        }}
      >
        <Tabs.Screen
          name="index"
          options={{
            title: 'Review',
            tabBarIcon: ({ color, size }) => (
              <Ionicons name="scan-outline" color={typeof color === 'string' ? color : colors.accent} size={size} />
            ),
          }}
        />
        <Tabs.Screen
          name="scope"
          options={{
            title: 'Scope',
            tabBarIcon: ({ color, size }) => (
              <Ionicons name="shield-outline" color={typeof color === 'string' ? color : colors.accent} size={size} />
            ),
          }}
        />
      </Tabs>
    </>
  );
}
