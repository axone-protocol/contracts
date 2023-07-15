import type { NextPage } from "next";
import Head from "next/head";
import { useState } from "react";
import { useCount } from "../api/counter";
import styles from "../styles/Home.module.css";

const Home: NextPage = () => {
  const { count, error, increase } = useCount();
  const [isLoading, setLoading] = useState(false);

  return (
    <div className={styles.container}>
      <Head>
        <title>ObjectiFi</title>
        <meta name="description" content="Counter dapp: an example dapp" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>ObjectiFi</h1>
        <p style={{ fontSize: "24px" }}>
          ObjectiFi uses the Objectarium and cw721 smartcontracts to turn uploaded files 
          into NFTs. These Objects are then searcheable on the OKP4 explorer
        </p>
        <p
          className={
            isLoading ? [styles.count, styles.pulse].join(" ") : styles.count
          }
        >
          <div className={styles.grid}>
            <a
              className={styles.card}
              onClick={async () => {
                setLoading(true);
                await increase();
                setLoading(false);
              }}
            >
              <h2>Create NFT</h2>
            </a>
          </div>
        </p>
        <div className={styles.grid}>
          <a
            className={styles.card}
            onClick={async () => {
              setLoading(true);
              await increase();
              setLoading(false);
            }}
          >
            <h2>Query Objectarium</h2>
          </a>
        </div>
      </main>
    </div>
  );
};

export default Home;
