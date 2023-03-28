import { useState, createContext } from "react";
import type React, { ReactNode } from "react";

const initial: IFrameRateContext = {
  frameRate: 0,
  setFrameRate: _ => {},
};

export const FrameRateContext = createContext(initial);

export const FrameRateContextProvider: React.FC<FrameRateContextProviderProps> = ({
  children
}) => {
  const [frameRate, setFrameRate] = useState<number>(0);
  const value = { frameRate, setFrameRate };

  return (
    <FrameRateContext.Provider value={value}>
      {children}
    </FrameRateContext.Provider>
  );
};

interface IFrameRateContext {
  frameRate: number
  setFrameRate: (_: number) => void
}

interface FrameRateContextProviderProps {
  children: ReactNode
}
