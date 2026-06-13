// Первым — полифилл Buffer (Anchor/borsh требуют глобальный Buffer до своего импорта)
import "./buffer-polyfill";

import React from "react";
import ReactDOM from "react-dom/client";
import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-phantom";
import App from "./App";

// Стили кошелька (кнопка Connect Wallet)
import "@solana/wallet-adapter-react-ui/styles.css";

// ===================================================================
// НАСТРОЙКА СЕТИ
// Для работы с локальным валидатором: http://127.0.0.1:8899
// Для Devnet: https://api.devnet.solana.com
// ===================================================================
const ENDPOINT = "http://127.0.0.1:8899"; // Localnet по умолчанию

const wallets = [new PhantomWalletAdapter()];

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ConnectionProvider endpoint={ENDPOINT}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <App />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  </React.StrictMode>
);
