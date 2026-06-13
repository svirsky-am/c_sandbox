import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { expect } from "chai";

// ===========================================================
// Вспомогательная функция: вычислить PDA счётчика по кошельку
// ===========================================================
function getCounterPDA(
  authority: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), authority.toBuffer()],
    programId
  );
}

// ===========================================================
// Вспомогательная функция: создать изолированный кошелёк с SOL
// Каждый тест получает свой authority — тесты независимы друг от друга
// ===========================================================
async function createFundedAuthority(
  provider: AnchorProvider
): Promise<anchor.web3.Keypair> {
  const authority = anchor.web3.Keypair.generate();
  const sig = await provider.connection.requestAirdrop(
    authority.publicKey,
    2 * anchor.web3.LAMPORTS_PER_SOL
  );
  const latestBlockhash = await provider.connection.getLatestBlockhash();
  await provider.connection.confirmTransaction({
    signature: sig,
    ...latestBlockhash,
  });
  return authority;
}

// ===========================================================
// Тесты
// ===========================================================

describe("counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Counter as Program<Counter>;

  // -----------------------------------------------------------
  // Тест 1: Инициализация
  // -----------------------------------------------------------
  it("✅ initializes counter with count = 0", async () => {
    const authority = await createFundedAuthority(provider);
    const [counterPDA] = getCounterPDA(authority.publicKey, program.programId);

    await program.methods
      .initialize()
      .accounts({
        counter: counterPDA,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    const account = await program.account.counter.fetch(counterPDA);

    // Начальное значение должно быть 0
    expect(account.count.toNumber()).to.equal(0);
    // Authority должен совпадать с создателем
    expect(account.authority.toString()).to.equal(
      authority.publicKey.toString()
    );
  });

  // -----------------------------------------------------------
  // Тест 2: Одиночный инкремент
  // -----------------------------------------------------------
  it("✅ increments counter from 0 to 1", async () => {
    const authority = await createFundedAuthority(provider);
    const [counterPDA] = getCounterPDA(authority.publicKey, program.programId);

    // Сначала инициализируем
    await program.methods
      .initialize()
      .accounts({
        counter: counterPDA,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    // Затем инкрементируем
    await program.methods
      .increment()
      .accounts({
        counter: counterPDA,
        authority: authority.publicKey,
      })
      .signers([authority])
      .rpc();

    const account = await program.account.counter.fetch(counterPDA);
    expect(account.count.toNumber()).to.equal(1);
  });

  // -----------------------------------------------------------
  // Тест 3: Множественный инкремент
  // -----------------------------------------------------------
  it("✅ increments counter multiple times correctly", async () => {
    const authority = await createFundedAuthority(provider);
    const [counterPDA] = getCounterPDA(authority.publicKey, program.programId);
    const INCREMENTS = 7;

    await program.methods
      .initialize()
      .accounts({
        counter: counterPDA,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    for (let i = 0; i < INCREMENTS; i++) {
      await program.methods
        .increment()
        .accounts({
          counter: counterPDA,
          authority: authority.publicKey,
        })
        .signers([authority])
        .rpc();
    }

    const account = await program.account.counter.fetch(counterPDA);
    expect(account.count.toNumber()).to.equal(INCREMENTS);
  });

  // -----------------------------------------------------------
  // Тест 4: PDA независим для каждого кошелька
  // -----------------------------------------------------------
  it("✅ each wallet has its own independent counter", async () => {
    const alice = await createFundedAuthority(provider);
    const bob = await createFundedAuthority(provider);

    const [alicePDA] = getCounterPDA(alice.publicKey, program.programId);
    const [bobPDA] = getCounterPDA(bob.publicKey, program.programId);

    // Инициализируем оба счётчика
    await program.methods
      .initialize()
      .accounts({
        counter: alicePDA,
        authority: alice.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([alice])
      .rpc();

    await program.methods
      .initialize()
      .accounts({
        counter: bobPDA,
        authority: bob.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([bob])
      .rpc();

    // Alice инкрементирует 3 раза
    for (let i = 0; i < 3; i++) {
      await program.methods
        .increment()
        .accounts({ counter: alicePDA, authority: alice.publicKey })
        .signers([alice])
        .rpc();
    }

    // Bob инкрементирует 1 раз
    await program.methods
      .increment()
      .accounts({ counter: bobPDA, authority: bob.publicKey })
      .signers([bob])
      .rpc();

    const aliceAccount = await program.account.counter.fetch(alicePDA);
    const bobAccount = await program.account.counter.fetch(bobPDA);

    expect(aliceAccount.count.toNumber()).to.equal(3);
    expect(bobAccount.count.toNumber()).to.equal(1);
  });

  // -----------------------------------------------------------
  // Тест 5: Нельзя инкрементировать чужой счётчик
  // -----------------------------------------------------------
  it("❌ rejects increment from wrong authority", async () => {
    const alice = await createFundedAuthority(provider);
    const mallory = await createFundedAuthority(provider);

    const [alicePDA] = getCounterPDA(alice.publicKey, program.programId);

    await program.methods
      .initialize()
      .accounts({
        counter: alicePDA,
        authority: alice.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([alice])
      .rpc();

    // Mallory пытается инкрементировать счётчик Alice
    // Anchor проверяет seeds — адрес PDA не совпадёт с alicePDA
    try {
      await program.methods
        .increment()
        .accounts({
          counter: alicePDA,       // счётчик Alice
          authority: mallory.publicKey, // но подписывает Mallory
        })
        .signers([mallory])
        .rpc();

      // Если сюда добрались — тест должен упасть
      expect.fail("Expected transaction to fail but it succeeded");
    } catch (err: any) {
      // Ожидаем ошибку: seeds constraint или has_one constraint
      expect(err).to.be.instanceOf(Error);
    }
  });

  // ===========================================================
  // ДОМАШНЕЕ ЗАДАНИЕ — раскомментируйте после добавления decrement
  // ===========================================================

  // it("✅ [BONUS] decrements counter", async () => {
  //   const authority = await createFundedAuthority(provider);
  //   const [counterPDA] = getCounterPDA(authority.publicKey, program.programId);
  //
  //   await program.methods
  //     .initialize()
  //     .accounts({
  //       counter: counterPDA,
  //       authority: authority.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([authority])
  //     .rpc();
  //
  //   // Инкрементируем до 3
  //   for (let i = 0; i < 3; i++) {
  //     await program.methods
  //       .increment()
  //       .accounts({ counter: counterPDA, authority: authority.publicKey })
  //       .signers([authority])
  //       .rpc();
  //   }
  //
  //   // Декрементируем до 1
  //   for (let i = 0; i < 2; i++) {
  //     await program.methods
  //       .decrement()
  //       .accounts({ counter: counterPDA, authority: authority.publicKey })
  //       .signers([authority])
  //       .rpc();
  //   }
  //
  //   const account = await program.account.counter.fetch(counterPDA);
  //   expect(account.count.toNumber()).to.equal(1);
  // });

  // it("❌ [BONUS] fails to decrement below zero", async () => {
  //   const authority = await createFundedAuthority(provider);
  //   const [counterPDA] = getCounterPDA(authority.publicKey, program.programId);
  //
  //   await program.methods
  //     .initialize()
  //     .accounts({
  //       counter: counterPDA,
  //       authority: authority.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([authority])
  //     .rpc();
  //
  //   try {
  //     // Счётчик = 0, декремент должен вернуть ошибку BelowZero
  //     await program.methods
  //       .decrement()
  //       .accounts({ counter: counterPDA, authority: authority.publicKey })
  //       .signers([authority])
  //       .rpc();
  //
  //     expect.fail("Expected BelowZero error");
  //   } catch (err: any) {
  //     expect(err.error.errorCode.code).to.equal("BelowZero");
  //   }
  // });
});
