import { useState, useEffect, useCallback } from "react";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { Program, AnchorProvider, BN } from "@coral-xyz/anchor";
import type anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import idl from "./idl/counter.json";
import "./App.css";

// ===================================================================
// ОБНОВИТЕ Program ID после выполнения `anchor build` и `anchor deploy`
// Скопируйте значение из `anchor keys list` или из Anchor.toml
// ===================================================================
const PROGRAM_ID = new PublicKey(
  (idl as { address?: string }).address ??
    "11111111111111111111111111111111" // fallback (System Program — означает IDL не настроен)
);

// ===================================================================
// Вычислить PDA счётчика для данного кошелька
// Логика совпадает с seeds = [b"counter", authority] в lib.rs
// ===================================================================
function getCounterPDA(walletPubkey: PublicKey): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), walletPubkey.toBuffer()],
    PROGRAM_ID
  );
  return pda;
}

// ===================================================================
// Типы для удобства
// ===================================================================
interface TxEntry {
  action: string;
  sig: string;
  ts: string;
}

// ===================================================================
// Главный компонент
// ===================================================================
// По RPC URL определить понятное имя сети (для отображения в UI)
function getNetworkLabel(rpcUrl: string): string {
  if (!rpcUrl) return "—";
  if (/127\.0\.0\.1|localhost|8899/.test(rpcUrl)) return "Localhost";
  if (/devnet\.solana|api\.devnet\.solana/.test(rpcUrl)) return "Devnet";
  if (/mainnet|api\.mainnet/.test(rpcUrl)) return "Mainnet";
  return new URL(rpcUrl).hostname || rpcUrl.slice(0, 24) + "…";
}

export default function App() {
  const { connection } = useConnection();
  const wallet = useWallet();
  const networkLabel = getNetworkLabel(connection.rpcEndpoint);

  const [count, setCount] = useState<number | null>(null);
  const [counterPDA, setCounterPDA] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [txLog, setTxLog] = useState<TxEntry[]>([]);
  const [error, setError] = useState<string>("");

  // Получить экземпляр программы Anchor
  const getProgram = useCallback(() => {
    if (!wallet.publicKey || !wallet.signTransaction) return null;
    const provider = new AnchorProvider(connection, wallet as unknown as anchor.Wallet, {
      commitment: "confirmed",
    });
    return new Program(idl as unknown as anchor.Idl, provider);
  }, [connection, wallet]);

  // Прочитать текущее значение счётчика из аккаунта
  const fetchCount = useCallback(async (pda: PublicKey) => {
    const program = getProgram();
    if (!program) return;
    try {
      const account = await (program.account as Record<string, { fetch: (pda: PublicKey) => Promise<{ count: BN }> }>)["counter"].fetch(pda);
      setCount(account.count.toNumber());
      setError("");
    } catch {
      // Аккаунт не существует — счётчик не инициализирован
      setCount(null);
    }
  }, [getProgram]);

  // При подключении кошелька — вычислить PDA и попытаться прочитать счётчик
  useEffect(() => {
    if (wallet.publicKey) {
      const pda = getCounterPDA(wallet.publicKey);
      setCounterPDA(pda.toString());
      fetchCount(pda);
    } else {
      setCount(null);
      setCounterPDA("");
      setTxLog([]);
    }
  }, [wallet.publicKey, fetchCount]);

  // Добавить транзакцию в лог
  function logTx(action: string, sig: string) {
    const entry: TxEntry = {
      action,
      sig,
      ts: new Date().toLocaleTimeString(),
    };
    setTxLog((prev) => [entry, ...prev].slice(0, 6));
  }

  // ----------------------------------------------------------------
  // Инициализация счётчика
  // ----------------------------------------------------------------
  async function handleInitialize() {
    if (!wallet.publicKey) return;
    const program = getProgram();
    if (!program) return;

    setLoading(true);
    setError("");
    try {
      const pda = getCounterPDA(wallet.publicKey);
      const sig = await (program.methods as any)
        .initialize()
        .accounts({
          counter: pda,
          authority: wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      logTx("Initialize", sig);
      await fetchCount(pda);
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
    } finally {
      setLoading(false);
    }
  }

  // ----------------------------------------------------------------
  // Инкремент
  // ----------------------------------------------------------------
  async function handleIncrement() {
    if (!wallet.publicKey) return;
    const program = getProgram();
    if (!program) return;

    setLoading(true);
    setError("");
    try {
      const pda = getCounterPDA(wallet.publicKey);
      const sig = await (program.methods as any)
        .increment()
        .accounts({
          counter: pda,
          authority: wallet.publicKey,
        })
        .rpc();

      logTx("Increment", sig);
      await fetchCount(pda);
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
    } finally {
      setLoading(false);
    }
  }

  const isConnected = !!wallet.publicKey;
  const isInitialized = count !== null;

  return (
    <div className="app">
      {/* ---- Шапка ---- */}
      <header className="header">
        <div className="header-title">
          <span className="logo">⛓</span>
          <div>
            <h1>Counter Dev-UI</h1>
            <p className="subtitle">Solana Workshop #5</p>
          </div>
        </div>
        <WalletMultiButton />
      </header>

      {!isConnected ? (
        /* ---- Нет кошелька ---- */
        <div className="empty-state">
          <div className="empty-icon">👜</div>
          <h2>Подключите Phantom</h2>
          <p>
            Убедитесь, что Phantom переключён на{" "}
            <strong>Localhost (127.0.0.1:8899)</strong>
          </p>
          <p className="hint">
            Phantom → Settings → Developer Settings → Custom RPC URL
          </p>
        </div>
      ) : (
        <main className="main">
          {/* ---- Инфо о кошельке ---- */}
          <div className="card info-card">
            <div className="info-row">
              <span className="label">Wallet</span>
              <span className="mono addr">{wallet.publicKey!.toString()}</span>
            </div>
            <div className="info-row">
              <span className="label">Counter PDA</span>
              <span className="mono addr small">{counterPDA}</span>
            </div>
          </div>

          {/* ---- Дисплей счётчика ---- */}
          <div className="card counter-card">
            <p className="label">Текущее значение</p>
            <div className={`count-display ${isInitialized ? "active" : "inactive"}`}>
              {isInitialized ? count : "—"}
            </div>
            {!isInitialized && (
              <p className="hint">Счётчик не инициализирован. Нажмите Initialize.</p>
            )}
          </div>

          {/* ---- Кнопки ---- */}
          <div className="actions">
            <button
              className="btn btn-init"
              onClick={handleInitialize}
              disabled={loading || isInitialized}
              title={isInitialized ? "Счётчик уже инициализирован" : ""}
            >
              {loading && !isInitialized ? "⏳" : "🚀"} Initialize
            </button>

            <button
              className="btn btn-inc"
              onClick={handleIncrement}
              disabled={loading || !isInitialized}
            >
              {loading && isInitialized ? "⏳" : "+"} Increment
            </button>

            <button
              className="btn btn-refresh"
              onClick={() => isConnected && fetchCount(new PublicKey(counterPDA))}
              disabled={loading || !counterPDA}
            >
              ↻ Refresh
            </button>
          </div>

          {/* ---- Ошибка ---- */}
          {error && (
            <div className="card error-card">
              <p className="label">Ошибка</p>
              <p className="mono small">{error}</p>
            </div>
          )}

          {/* ---- Лог транзакций ---- */}
          {txLog.length > 0 && (
            <div className="card tx-log">
              <p className="label">Последние транзакции</p>
              <div className="tx-list">
                {txLog.map((tx, i) => (
                  <div key={i} className="tx-entry">
                    <span className="tx-action">{tx.action}</span>
                    <span className="mono small tx-sig">
                      {tx.sig.slice(0, 8)}…{tx.sig.slice(-6)}
                    </span>
                    <span className="tx-ts">{tx.ts}</span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </main>
      )}

      <footer className="footer">
        <span title={connection.rpcEndpoint}>
          Сеть: <strong>{networkLabel}</strong>
        </span>
        {" · "}
        Program ID: <span className="mono">{PROGRAM_ID.toString().slice(0, 16)}…</span>
      </footer>
    </div>
  );
}
