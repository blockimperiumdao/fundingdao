use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupSet, UnorderedMap};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::BorshStorageKey;

use strum::IntoEnumIterator;

use strum::{EnumIter};


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone, EnumIter)]
#[serde(crate = "near_sdk::serde")]
pub enum ApplicationType{
    Grant = 1,
    VentureCaptital = 2,
} 

#[derive(BorshStorageKey, BorshSerialize)]
pub enum GrantApplicationStorageKeys {
    AdminWallets,
    GrantApplications
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone, EnumIter)]
#[serde(crate = "near_sdk::serde")]
pub enum GrantCategoryType{
    ChannelBrandPartnership = 1,
    Dao = 2,
    GamingMetaverse = 3,
    InfrastructureWallets = 4,
    InstitutionalFinancial = 5,
    Nft = 6,
    Other = 7,
    SocialImpact = 8
} 

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone, EnumIter)]
#[serde(crate = "near_sdk::serde")]
pub enum ApplicantCountry{
    UnitedStates = 1,
    Afghanistan = 2,
    Albania = 3,
    Algeria = 4,
    AmericanSamoa = 5,
    Andorra = 6,
    Angola = 7,
    Anguilla = 8,
    Antartica =9, 
} 

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone, EnumIter)]
#[serde(crate = "near_sdk::serde")]
pub enum ApplicationStatus{
    Submitted = 1,
    Rejected = 2,
    InReview = 3
} 

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, )]
#[serde(crate = "near_sdk::serde")]
pub struct Feedback {
    account: AccountId,
    description: String,
    feedback_date: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct GrantApplication {
    id: String,
    applicant: AccountId,
    project_name: String,
    application_type: ApplicationType,
    grant_category: GrantCategoryType,
    description: String,
    applicant_first_name: String,
    applicant_last_name: String,
    applicant_contact_email: String,
    applicant_country: ApplicantCountry,
    applicant_address_line1: String,
    applicant_address_line2: String,
    applicant_city: String,
    applicant_state: String,
    applicant_zipcode: String,
    team_size: u8,
    team_description: String,
    team_github: String,
    project_website: String,
    project_github: String,
    grant_size_requested: u32,
    project_phase: String,
    target_launch_date: u64,
    project_purpose: String,
    project_competitive_advantage: String,
    project_milestones: String,
    project_is_opensource: bool,
    project_opensource_components: String,
    project_ecosystem_impact: String,
    project_near_excitement: String,
    project_success_criteria: String,
    project_pitch_video_url: String,
    project_pitch_deck_url: String,

    application_status: ApplicationStatus,
}

impl GrantApplication {
    pub fn new(
        project_name: String,
        application_type: ApplicationType, 
        grant_category: GrantCategoryType,
        description: String,
        applicant_first_name: String,
        applicant_last_name: String,
        applicant_contact_email: String,
        applicant_country: ApplicantCountry,
        applicant_address_line1: String,
        applicant_address_line2: String,
        applicant_city: String,
        applicant_state: String,
        applicant_zipcode: String,
        team_size: u8,
        team_description: String,
        team_github: String,
        project_website: String,
        project_github: String,
        grant_size_requested: u32,
        project_phase: String,
        target_launch_date: u64,
        project_purpose: String,
        project_competitive_advantage: String,
        project_milestones: String,
        project_is_opensource: bool,
        project_opensource_components: String,
        project_ecosystem_impact: String,
        project_near_excitement: String,
        project_success_criteria: String,
        project_pitch_video_url: String,
        project_pitch_deck_url: String,
    
        application_status: ApplicationStatus,
    ) -> GrantApplication 
    {
        GrantApplication {
            id: env::block_timestamp().to_string(),
            applicant: env::signer_account_id(),
            project_name: project_name,
            application_type: application_type,
            grant_category: grant_category,
            description: description,
            applicant_first_name: applicant_first_name,
            applicant_last_name: applicant_last_name,
            applicant_contact_email: applicant_contact_email,
            applicant_country: applicant_country,
            applicant_address_line1: applicant_address_line1,
            applicant_address_line2: applicant_address_line2,
            applicant_city: applicant_city,
            applicant_state: applicant_state,
            applicant_zipcode: applicant_zipcode,
            team_size: team_size,
            team_description: team_description,
            team_github: team_github,
            project_website: project_website,
            project_github: project_github,
            grant_size_requested: grant_size_requested,
            project_phase: project_phase,
            target_launch_date: target_launch_date,
            project_purpose: project_purpose,
            project_competitive_advantage: project_competitive_advantage,
            project_milestones: project_milestones,
            project_is_opensource: project_is_opensource,
            project_opensource_components: project_opensource_components,
            project_ecosystem_impact: project_ecosystem_impact,
            project_near_excitement: project_near_excitement,
            project_success_criteria: project_success_criteria,
            project_pitch_video_url: project_pitch_video_url,
            project_pitch_deck_url: project_pitch_deck_url,
    
            application_status: application_status,
        }
    }

    pub fn update( &mut self,
        project_name: String,
        application_type: ApplicationType, 
        grant_category: GrantCategoryType,
        description: String,
        applicant_first_name: String,
        applicant_last_name: String,
        applicant_contact_email: String,
        applicant_country: ApplicantCountry,
        applicant_address_line1: String,
        applicant_address_line2: String,
        applicant_city: String,
        applicant_state: String,
        applicant_zipcode: String,
        team_size: u8,
        team_description: String,
        team_github: String,
        project_website: String,
        project_github: String,
        grant_size_requested: u32,
        project_phase: String,
        target_launch_date: u64,
        project_purpose: String,
        project_competitive_advantage: String,
        project_milestones: String,
        project_is_opensource: bool,
        project_opensource_components: String,
        project_ecosystem_impact: String,
        project_near_excitement: String,
        project_success_criteria: String,
        project_pitch_video_url: String,
        project_pitch_deck_url: String, 
        application_status: ApplicationStatus,
        )
    {
        self.project_name = project_name;
        self.application_type= application_type;
        self.grant_category= grant_category;
        self.description= description;
        self.applicant_first_name= applicant_first_name;
        self.applicant_last_name= applicant_last_name;
        self.applicant_contact_email= applicant_contact_email;
        self.applicant_country= applicant_country;
        self.applicant_address_line1= applicant_address_line1;
        self.applicant_address_line2= applicant_address_line2;
        self.applicant_city= applicant_city;
        self.applicant_state= applicant_state;
        self.applicant_zipcode= applicant_zipcode;
        self.team_size= team_size;
        self.team_description= team_description;
        self.team_github= team_github;
        self.project_website= project_website;
        self.project_github= project_github;
        self.grant_size_requested= grant_size_requested;
        self.project_phase= project_phase;
        self.target_launch_date= target_launch_date;
        self.project_purpose= project_purpose;
        self.project_competitive_advantage= project_competitive_advantage;
        self.project_milestones= project_milestones;
        self.project_is_opensource= project_is_opensource;
        self.project_opensource_components= project_opensource_components;
        self.project_ecosystem_impact= project_ecosystem_impact;
        self.project_near_excitement= project_near_excitement;
        self.project_success_criteria= project_success_criteria;
        self.project_pitch_video_url= project_pitch_video_url;
        self.project_pitch_deck_url= project_pitch_deck_url;

        self.application_status= application_status;
    }

}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub admin_list: LookupSet<AccountId>,
    pub grant_applications: UnorderedMap<AccountId, GrantApplication>,
}


#[near_bindgen]
impl Contract {

    /// initializes the contract 
    #[init]
    pub fn new() -> Self 
    {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            admin_list: LookupSet::new(GrantApplicationStorageKeys::AdminWallets),
            grant_applications: UnorderedMap::new(GrantApplicationStorageKeys::GrantApplications)
        }
    }

    pub fn get_application_types (&self) -> Vec<ApplicationType>
    {
        let application_types: Vec<_> = ApplicationType::iter().collect();

        return application_types;
    }   
    
    pub fn get_category_types (&self) -> Vec<GrantCategoryType>
    {
        let category_types: Vec<_> = GrantCategoryType::iter().collect();

        return category_types;
    } 

    pub fn get_applicant_countries (&self) -> Vec<ApplicantCountry>
    {
        let applicant_countries: Vec<_> = ApplicantCountry::iter().collect();

        return applicant_countries;
    }

    pub fn get_application_status_types (&self) -> Vec<ApplicationStatus>
    {
        let application_status_types: Vec<_> = ApplicationStatus::iter().collect();

        return application_status_types;
    }

    #[private]
    pub fn add_admin(&mut self, 
                        account: AccountId)
    {
        log!("Adding admin wallet {}", account );
        self.admin_list.insert(&account);
    }

    #[private]
    pub fn remove_admin(&mut self,
                            account: AccountId )
    {
        log!("Removing admin wallet {}", account );
        self.admin_list.remove(&account);
    }

    pub fn admin_check(&self)
    {
        if ! self.admin_list.contains( &env::signer_account_id() )
        {
            env::panic_str("Not permitted")    
        }
    }

    pub fn add_application(&mut self,
                            grant_application: GrantApplication, )
    {
        let signer = env::signer_account_id();
        self.grant_applications.insert(&signer, &grant_application);
    }

    pub fn update_application_status(&mut self, 
                                        account_id: AccountId,
                                        status: ApplicationStatus)
    {
        self.admin_check();

        let mut grant_application = self.grant_applications.get(&account_id).unwrap();
        grant_application.application_status = status;

        self.grant_applications.insert(&account_id, &grant_application);
    }

    pub fn get_grant_applications(&self, from_index: u64, limit: u64) -> Vec<(AccountId, GrantApplication)>
    {
        let keys = self.grant_applications.keys_as_vector();
        let values = self.grant_applications.values_as_vector();

        (from_index..std::cmp::min(from_index + limit, self.grant_applications.len()))
        .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
        .collect()  
    }

    pub fn get_grant_application(&self, account:AccountId) -> Option<GrantApplication>
    {
        return self.grant_applications.get(&account);
    }

    pub fn upsert_grant_application(&mut self,
        id: String, 
        account:AccountId,     
        project_name: String,
        application_type: ApplicationType, 
        grant_category: GrantCategoryType,
        description: String,
        applicant_first_name: String,
        applicant_last_name: String,
        applicant_contact_email: String,
        applicant_country: ApplicantCountry,
        applicant_address_line1: String,
        applicant_address_line2: String,
        applicant_city: String,
        applicant_state: String,
        applicant_zipcode: String,
        team_size: u8,
        team_description: String,
        team_github: String,
        project_website: String,
        project_github: String,
        grant_size_requested: u32,
        project_phase: String,
        target_launch_date: u64,
        project_purpose: String,
        project_competitive_advantage: String,
        project_milestones: String,
        project_is_opensource: bool,
        project_opensource_components: String,
        project_ecosystem_impact: String,
        project_near_excitement: String,
        project_success_criteria: String,
        project_pitch_video_url: String,
        project_pitch_deck_url: String,
    
        application_status: ApplicationStatus
       ) -> GrantApplication
    {
        
        if id.is_empty()
        {
            let grant_application = GrantApplication::new(
                project_name,
                application_type, 
                grant_category,
                description,
                applicant_first_name,
                applicant_last_name,
                applicant_contact_email,
                applicant_country,
                applicant_address_line1,
                applicant_address_line2,
                applicant_city,
                applicant_state,
                applicant_zipcode,
                team_size,
                team_description,
                team_github,
                project_website,
                project_github,
                grant_size_requested,
                project_phase,
                target_launch_date,
                project_purpose,
                project_competitive_advantage,
                project_milestones,
                project_is_opensource,
                project_opensource_components,
                project_ecosystem_impact,
                project_near_excitement,
                project_success_criteria,
                project_pitch_video_url,
                project_pitch_deck_url,
                application_status
            );

            self.grant_applications.insert( &account, &grant_application);

            return grant_application;
        }
        else
        {
            let mut grant_application = self.grant_applications.get(&account).unwrap();

            grant_application.update( 
                project_name,
                application_type, 
                grant_category,
                description,
                applicant_first_name,
                applicant_last_name,
                applicant_contact_email,
                applicant_country,
                applicant_address_line1,
                applicant_address_line2,
                applicant_city,
                applicant_state,
                applicant_zipcode,
                team_size,
                team_description,
                team_github,
                project_website,
                project_github,
                grant_size_requested,
                project_phase,
                target_launch_date,
                project_purpose,
                project_competitive_advantage,
                project_milestones,
                project_is_opensource,
                project_opensource_components,
                project_ecosystem_impact,
                project_near_excitement,
                project_success_criteria,
                project_pitch_video_url,
                project_pitch_deck_url,
                application_status
           );
           self.grant_applications.insert( &account, &grant_application );

           return grant_application;
        }
    }

}