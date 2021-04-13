use crate::entity::profile::{
    Profile, ProfileCreator, ProfileOps, ProfileSet, ProfileSetCreator, ProfileSetOps,
    RemoteProfile,
};
use crate::*;
use ockam_vault::SoftwareVault;
use ockam_vault_core::CompositeVault;
use rand::{thread_rng, CryptoRng, RngCore};

pub type EntityRng = Box<dyn Rng>;
pub type EntityVault = Box<dyn CompositeVault>;

pub struct SecureChannel; // TODO: place holder type

pub struct Entity {
    default_profile: ProfileIdentifier,
    profiles: ProfileSet,
    vault: EntityVault,
    rng: EntityRng,
}

pub trait SecureChannelCreator {
    fn secure_channel_create<P: ProfileOps, R: ProfileOps>(
        local: &P,
        _remote: &R,
    ) -> crate::Result<SecureChannel> {
        println!(
            "secure_channel_create: {:?} -> {:?}",
            local.profile_id(),
            _remote.profile_id()
        );
        todo!()
    }
}

impl ProfileOps for Entity {
    fn profile_id(&self) -> ProfileIdentifier {
        self.get_default_profile().profile_id()
    }
}

impl SecureChannelCreator for Entity {
    fn secure_channel_create<P: ProfileOps, R: ProfileOps>(
        _local: &P,
        _remote: &R,
    ) -> crate::Result<SecureChannel> {
        todo!()
    }
}

impl Issuer for Entity {
    fn get_signing_key(&self) -> SigningKeyBytes {
        todo!()
    }

    fn get_public_key(&self) -> PublicKeyBytes {
        todo!()
    }

    fn create_offer(&self, _schema: &CredentialSchema, _rng: impl Rng) -> CredentialOffer {
        todo!()
    }

    fn create_proof_of_possession(&self) -> ProofBytes {
        todo!()
    }

    fn sign_credential(
        &self,
        _schema: &CredentialSchema,
        _attributes: &[CredentialAttribute],
    ) -> Result<Credential> {
        todo!()
    }

    fn sign_credential_request(
        &self,
        _ctx: &CredentialRequest,
        _schema: &CredentialSchema,
        _attributes: &[(String, CredentialAttribute)],
        _offer_id: OfferIdBytes,
    ) -> Result<CredentialFragment2> {
        todo!()
    }
}

impl Holder for Entity {
    fn accept_credential_offer(
        &self,
        _offer: &CredentialOffer,
        _issuer_pk: PublicKeyBytes,
        _rng: impl RngCore + CryptoRng,
    ) -> Result<(CredentialRequest, CredentialFragment1)> {
        todo!()
    }

    fn combine_credential_fragments(
        &self,
        _credential_fragment1: CredentialFragment1,
        _credential_fragment2: CredentialFragment2,
    ) -> Credential {
        todo!()
    }

    fn is_valid_credential(
        &self,
        _credential: &Credential,
        _verifying_key: PublicKeyBytes,
    ) -> bool {
        todo!()
    }

    fn present_credentials(
        &self,
        _credential: &[Credential],
        _presentation_manifests: &[PresentationManifest],
        _proof_request_id: ProofRequestIdBytes,
        _rng: impl RngCore + CryptoRng,
    ) -> Result<Vec<CredentialPresentation>> {
        todo!()
    }
}

impl Entity {
    fn new(vault: EntityVault) -> Entity {
        let default_profile = "keyid";
        let default = Profile::new(ProfileIdentifier::from(default_profile.clone()));
        let profiles = ProfileSet::new(default.clone());
        Entity {
            rng: Box::new(thread_rng()),
            default_profile: default.profile_id(),
            profiles,
            vault,
        }
    }

    fn get_default_profile(&self) -> &Profile {
        if let Some(default) = self.profiles.get(self.default_profile.clone()) {
            default
        } else {
            panic!("Entity missing default profile")
        }
    }

    fn create_secure_channel(&self, remote: &RemoteProfile) -> Result<SecureChannel> {
        Entity::secure_channel_create(self, remote)
    }
}

#[test]
fn entity_test() {
    let vault = Box::new(SoftwareVault::new());
    let _alice = Entity::new(vault);
}
