import { useState, useEffect } from "react";

export const useWindowDimension = () => {
  const [dimension, setDimension] = useState({width: 0, height: 0, pixelRatio: 1});

  useEffect(() => {
    const resize = () => setDimension({
      width: window.innerWidth,
      height: window.innerHeight,
      pixelRatio: window.devicePixelRatio,
    });

    resize();

    const debouncedResizeHandler = debounce(() => {
      console.log("***** debounced resize"); // See the cool difference in console
      // console.log([window.innerWidth, window.innerHeight, window.devicePixelRatio]); // See the cool difference in console
      resize();
    }, 100); // 100ms

    window.addEventListener("resize", debouncedResizeHandler);

    return () => window.removeEventListener("resize", debouncedResizeHandler);

  }, []); // Note this empty array. this effect should run only on mount and unmount

  return dimension;
};

const debounce = (callback: () => void, wait: number) => {
  let timeout;
  return (...args) => {
    const context = this;
    clearTimeout(timeout);
    timeout = setTimeout(() => callback.apply(context, args), wait);
  };
};
