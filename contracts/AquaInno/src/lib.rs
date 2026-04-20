#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contract]
pub struct AquaFundContract;

#[contractimpl]
impl AquaFundContract {
    // Fungsi untuk donasi ke dalam pool
    pub fn donate(env: Env, donor: Address, amount: i128) {
        donor.require_auth();
        
        // Asumsi kita menggunakan token native XLM
        // Logika transfer dari donor ke kontrak akan diimplementasikan di sini
        // menggunakan antarmuka token Soroban.
        
        // Simpan total donasi donor di storage
        let mut current_donation: i128 = env.storage().persistent().get(&donor).unwrap_or(0);
        current_donation += amount;
        env.storage().persistent().set(&donor, &current_donation);
    }

    // Fungsi untuk melihat total donasi seseorang
    pub fn get_donation(env: Env, donor: Address) -> i128 {
        env.storage().persistent().get(&donor).unwrap_or(0)
    }
}