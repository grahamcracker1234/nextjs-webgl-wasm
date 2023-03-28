import { useRef, useContext } from "react";
import { useWindowDimension } from "../hooks/useWindowDimension";
import { useAnimationFrame } from "../hooks/useAnimationFrame";
import { WASMContext } from "../context/WASM";
import { FrameRateContext } from "../context/FrameRate";

export const Canvas = ({ backgroundColor }) => {
  const { width, height, pixelRatio } = useWindowDimension();
  const canvas = useRef(null);
  const { wasm } = useContext(WASMContext);
  const { frameRate, setFrameRate } = useContext(FrameRateContext);

  useAnimationFrame(() => {
    const start = performance.now();

    if (wasm == null) return;
    wasm.render(canvas.current.getContext("webgl2"), width * pixelRatio, height * pixelRatio);

    const newFrameRate = 1000 / (performance.now() - start);
    if (isFinite(newFrameRate)) {
      const smoothing = 0.1;
      const smoothedFrameRate = newFrameRate * smoothing + frameRate * (1 - smoothing);
      setFrameRate(smoothedFrameRate);
    }
  });

	return (
    <div className="absolute inset-0 -z-10 mx-auto flex items-center justify-center" style={{ backgroundColor }}>
      <canvas 
        width={width * pixelRatio} 
        height={height * pixelRatio} 
        style={{width, height}}
        ref={canvas}>
      </canvas>
    </div>
  );
};
