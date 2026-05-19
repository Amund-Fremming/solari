const merchantIdentifier =
  process.env.EXPO_PUBLIC_STRIPE_MERCHANT_IDENTIFIER ||
  "merchant.com.solari.test";

/** @type {import('expo/config').ExpoConfig} */
module.exports = {
  expo: {
    name: "expo-test",
    slug: "expo-test",
    entryPoint: "./index.js",
    scheme: "solari-expo-test",
    version: "1.0.0",
    orientation: "portrait",
    userInterfaceStyle: "automatic",
    plugins: [
      "expo-asset",
      [
        "@stripe/stripe-react-native",
        {
          merchantIdentifier,
          enableGooglePay: false,
        },
      ],
    ],
  },
};
