export default function config() {
  return {
    isCI: !!process.env.CI,
    isProduction: process.env.NODE_ENV === "production",
  };
}
