// use sp_core::sr25519::Pair as Sr25519Pair;
use sp_core::Pair;
use std::collections::HashMap
use schnorrkel::{self,PublicKey, vrf::{VRFPreOut, VRFProof}};

#[derive(Clone)]
struct Commitment{
    msg: Vec<u8>,
    vrf_pre_out: VRFPreOut,
    vrf_proof: VRFProof,
}
struct Player {
    // x: f64,
    // y: f64,
    keypair: schnorrkel::keys::Keypair,
    commitments: Vec<Commitment>,
}

impl Player {
    fn new() -> Player {
        Player { 
            keypair: {
                let mut csprng = rand_core::OsRng;
                schnorrkel::keys::Keypair::generate_with(&mut csprng)
            },
            commitments: Vec::new(),
        }
    }

    fn draw(&mut self) {
        //let mut csprng = rand_core::OsRng;
        //let keypair1 = schnorrkel::keys::Keypair::generate_with(&mut csprng);

        let ctx =  schnorrkel::context::SigningContext::new(b"yo!");
        let msg = b"meow";
        let (io1, proof1, proof1batchable) = self.keypair.vrf_sign(ctx.bytes(msg));
        let out1: &VRFPreOut = &io1.to_preout();

        self.commitments.push(Commitment{
            vrf_pre_out: *out1,
            vrf_proof: proof1,
            msg: msg.to_vec(),
        });

    }

    fn reveal_all(&self) -> Vec<Commitment>{
        let mut new_commitments = Vec::new();
        new_commitments.clone_from_slice(&self.commitments);

        new_commitments
    }

    fn check_player(player_public_key: PublicKey, commitments: Vec<Commitment>){
        for commitment in commitments{
            player_public_key.vrf_verify(commitment.msg, &commitment.vrf_pre_out, &commitment.vrf_proof);
        }

    }
}

fn main() {
    // Let's assume two players

    // create two keypairs
    let alice: Player = Player::new();
    let bob: Player = Player::new();

    // create a player struct
    // player has keypair, draw method(create commitment), reveal_all method (show vrf proof)
}
