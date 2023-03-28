import type { NextPage } from "next";
import Head from "next/head";
import { Canvas } from "../components/Canvas";
import { FrameRate } from "../components/FrameRate";
import colors from "tailwindcss/colors";

const Home: NextPage = () => {
  return <>
    <Head>
      <title>Multidimensional Renderer</title>
      <meta name="description" content="Multidimensional Renderer"/>
      <link rel="icon" href="/favicon.ico"/>
    </Head>
    <Canvas backgroundColor={colors.black}/>
    <FrameRate/>
  </>;
};

export default Home;
