use anchor_lang::prelude::*;

// Program ID из dev-counter-keypair.json (копируется в target/deploy при make build / make deploy-localnet).
declare_id!("9Sq3k1ZXRNTFAMhjC8JrgUHn23dy13v2y56aV3tspMTt");

#[program]
pub mod counter {
    use super::*;

    /// Создаёт аккаунт счётчика для вызывающего кошелька.
    /// Адрес аккаунта — PDA, выведенный из публичного ключа authority.
    /// Можно вызвать только один раз для каждого кошелька.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.authority.key();
        counter.bump = ctx.bumps.counter;
        msg!("Counter initialized! Count: {}", counter.count);
        Ok(())
    }

    /// Увеличивает счётчик на 1.
    /// Может вызывать только владелец (authority) аккаунта.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter
            .count
            .checked_add(1)
            .ok_or(CounterError::Overflow)?;
        msg!("Counter incremented! New count: {}", counter.count);
        Ok(())
    }

    // =========================================================
    // ДОМАШНЕЕ ЗАДАНИЕ: реализовать decrement
    // =========================================================
    // Раскомментируйте функцию ниже и добавьте:
    //   1. Структуру Decrement (аналогично Increment)
    //   2. Ошибку BelowZero в CounterError
    //   3. Раскомментируйте BONUS-тесты в tests/counter.ts
    //
    // pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
    //     let counter = &mut ctx.accounts.counter;
    //     counter.count = counter
    //         .count
    //         .checked_sub(1)
    //         .ok_or(CounterError::BelowZero)?;
    //     msg!("Counter decremented! New count: {}", counter.count);
    //     Ok(())
    // }
}

// -----------------------------------------------------------------
// Контексты (наборы аккаунтов для каждой инструкции)
// -----------------------------------------------------------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// PDA-аккаунт счётчика.
    /// seeds = [b"counter", authority] — адрес выводится из кошелька,
    /// поэтому у каждого пользователя свой уникальный счётчик.
    #[account(
        init,
        payer = authority,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    /// Кошелёк, который создаёт и платит за аккаунт.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System Program нужна для создания нового аккаунта.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    /// Тот же PDA-аккаунт. Anchor проверяет, что seeds совпадают,
    /// тем самым гарантируя что мы меняем правильный счётчик.
    #[account(
        mut,
        seeds = [b"counter", authority.key().as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,

    /// Должен совпадать с authority, записанным в counter.authority.
    pub authority: Signer<'info>,
}

// ДОМАШНЕЕ ЗАДАНИЕ: добавьте структуру Decrement по аналогии с Increment

// -----------------------------------------------------------------
// Структура данных (хранится в аккаунте на блокчейне)
// -----------------------------------------------------------------

#[account]
#[derive(InitSpace)]
pub struct Counter {
    /// Текущее значение счётчика.
    pub count: u64,

    /// Публичный ключ владельца.
    pub authority: Pubkey,

    /// Bump seed для верификации PDA без дополнительных вычислений.
    pub bump: u8,
}

// -----------------------------------------------------------------
// Коды ошибок
// -----------------------------------------------------------------

#[error_code]
pub enum CounterError {
    #[msg("Counter overflow: reached maximum value")]
    Overflow,

    // ДОМАШНЕЕ ЗАДАНИЕ: добавьте ошибку BelowZero
    // #[msg("Counter cannot go below zero")]
    // BelowZero,
}
