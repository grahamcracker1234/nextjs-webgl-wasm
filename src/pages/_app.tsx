import "../styles/globals.css";
import type { AppProps } from "next/app";
import { WASMContextProvider } from "../context/WASM";
import { FrameRateContextProvider } from "../context/FrameRate";

const App = ({ Component, pageProps }: AppProps) => {
  return (
    <WASMContextProvider>
      <FrameRateContextProvider>
        <Component {...pageProps} />
      </FrameRateContextProvider>
    </WASMContextProvider>
  );
};

export default App;
