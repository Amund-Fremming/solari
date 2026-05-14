import { StatusBar } from "expo-status-bar";
import { SafeAreaView, Text } from "react-native";

export default function App() {
  return (
    <SafeAreaView
      style={{ flex: 1, alignItems: "center", justifyContent: "center" }}
    >
      <Text>solari expo-test starter</Text>
      <StatusBar style="auto" />
    </SafeAreaView>
  );
}
