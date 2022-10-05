use anchor_lang::prelude::*;
use crate::VoterId;

#[derive(Accounts)]
pub struct CreateVoterIdStruct<'info>{

    #[account(
        init,
        seeds = [
            b"orbit_voters"
        ],
        bump,
        payer = payer,
        space = 32
    )]
    pub voter_id_struct: Account<'info, VoterId>,

    #[account(
        mut
    )]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>
}

pub fn initialize_voter_id_handler(_ctx: Context<CreateVoterIdStruct>) -> Result<()>{
    Ok(())
}