import { useContext } from "react";
import { FrameRateContext } from "../context/FrameRate";

export const FrameRate = () => {
  const { frameRate } = useContext(FrameRateContext);

  return (
    <span className="ml-2 mt-1 inline-block text-white">{ Math.trunc(frameRate) }</span>
  );
};
