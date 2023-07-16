import type { NextPage } from "next";
import Head from "next/head";
import { useState } from "react";
import { useCount } from "../api/counter";
import styles from "../styles/Home.module.css";
import NFTModal from "./components/NFTModal";
import ObjectariumModal from "./components/ObjectariumModal";
import Button from '@mui/material/Button';
import React from "react";

const Home: NextPage = () => {
  const { count, error, increase } = useCount();
  const [isLoading, setLoading] = useState(false);
  const [nftOpen, nftSetOpen] = React.useState(false);
  const [objectariumOpen, objectariumSetOpen] = React.useState(false);
  const NFTModalOpen = () => nftSetOpen(true);
  const NFTModalClose = () => nftSetOpen(false);
  const ObjectariumModalOpen = () => objectariumSetOpen(true);
  const ObjectariumModalClose = () => objectariumSetOpen(false);

  const connectKeplr = async () => {
    if (!window.keplr) {
      alert("Please install keplr extension");
    } else {
      const chainId = "okp4-nemeton-1";

      // Enabling before using the Keplr is recommended.
      // This method will ask the user whether to allow access if they haven't visited this website.
      // Also, it will request that the user unlock the wallet if the wallet is locked.
      await window.keplr.enable(chainId);

      const offlineSigner = window.keplr.getOfflineSigner(chainId);

      // You can get the address/public keys by `getAccounts` method.
      // It can return the array of address/public key.
      // But, currently, Keplr extension manages only one address/public key pair.
      // XXX: This line is needed to set the sender address for SigningCosmosClient.
      const accounts = await offlineSigner.getAccounts();

      console.log("account: ", accounts)
    }
  }


  return (
    <>
      <header
        style={{
          textAlign: "right"
        }}
      >
        <Button variant="contained"
          style={{
            backgroundColor: "#0f224a",
            margin: "20px 20px 20px 20px",
            textAlign: "right"
          }}
          onClick={async () => {
            await connectKeplr()
          }}
        >Connect Wallet</Button>
      </header>
      <div className={styles.container}>
        <Head>
          <title>ObjectiFi</title>
          <meta name="description" content="Counter dapp: an example dapp" />
          <link rel="icon" href="/favicon.ico" />
        </Head>

        <main className={styles.main}>
          <h1 className={styles.title}>ObjectiFi</h1>
          <p style={{ fontSize: "24px", width: "1100px", textAlign: "center" }}>
            Mint any digital file as a new NFT and store it in Objectarium on the OKP4 Blockchain. The NFT objects in Objectarium can be queried, added, or removed from a Bucket or deleted entirely from the store.
          </p>
          <div className={styles.grid}>
            <a
              className={styles.card}
              onClick={() => {
                NFTModalOpen()
              }}
            >
              <h2>Create NFT</h2>
            </a>
          </div>

          <div className={styles.grid}>
            <a
              className={styles.card}
              onClick={() => {
                ObjectariumModalOpen()
              }}
            >
              <h2>Objectarium</h2>
            </a>
          </div>
        </main>
      </div>
      <NFTModal nftOpen={nftOpen} NFTModalClose={NFTModalClose}/>
      <ObjectariumModal objectariumOpen={objectariumOpen} ObjectariumModalClose={ObjectariumModalClose} />
    </>
  );
};

export default Home;
function makeCosmWasmClient(arg0: string, address: any) {
  throw new Error("Function not implemented.");
}

