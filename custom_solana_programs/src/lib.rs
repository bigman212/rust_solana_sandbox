pub mod hello_name_program {
    use std::str::FromStr;

    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        account_info::AccountInfo,
        entrypoint,
        entrypoint::ProgramResult,
        msg
    };
    use solana_program::pubkey::Pubkey;

    pub static PROGRAM_ID: Pubkey = solana_program::pubkey!("2BYjDV3FzQxLLAzyAtYq8Lzucet44bsSn8Gv2tvsz5BY");

    #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
    pub struct NamesData {
        pub group_leader: String,
        pub group_members: Vec<String>,
    }

    entrypoint!(hello_name_program);

    pub fn hello_name_program(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let decoded_data =
            NamesData::try_from_slice(&instruction_data)
                .unwrap();

        // log a message to the blockchain
        msg!("Hello, {}!", decoded_data.group_leader);
        decoded_data.group_members
            .iter()
            .for_each(|member_name| {
                msg!("Hello member, {}, welcome.", member_name)
            });

        // gracefully exit the program
        Ok(())
    }
}



