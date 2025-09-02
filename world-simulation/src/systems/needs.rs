use crate::tables::*;
use crate::types::*;
use crate::systems::modifiers::*;

impl Individual {
    /// Check if a higher level need is active (lower level must be adequate)
    pub fn is_need_level_active(&self, level: u8) -> bool {
        match level {
            1 => true, // Level 1 is always active
            2 => self.get_level_1_adequacy() >= thresholds::NEED_ADEQUATE,
            3 => self.is_need_level_active(2) && self.get_level_2_adequacy() >= thresholds::NEED_ADEQUATE,
            4 => self.is_need_level_active(3) && self.get_level_3_adequacy() >= thresholds::NEED_ADEQUATE,
            5 => self.is_need_level_active(4) && self.get_level_4_adequacy() >= thresholds::NEED_ADEQUATE,
            _ => false,
        }
    }
    
    /// Get average adequacy of Level 1 needs
    pub fn get_level_1_adequacy(&self) -> f32 {
        (self.food_water + self.environment + self.intimacy + self.rest + (100.0 - self.waste)) / 5.0
    }
    
    /// Get average adequacy of Level 2 needs
    pub fn get_level_2_adequacy(&self) -> f32 {
        (self.safety + (100.0 - self.threat) + self.safety + (100.0 - self.stress)) / 4.0
    }
    
    /// Get average adequacy of Level 3 needs
    pub fn get_level_3_adequacy(&self) -> f32 {
        (self.relationship + self.social_interaction + self.community) / 3.0
    }
    
    /// Get Level 4 adequacy (achievements)
    pub fn get_level_4_adequacy(&self) -> f32 {
        self.achievements
    }
    
    /// Update all needs based on time passed and current status
    pub fn update_needs(&mut self, hours_passed: u64, location: &LocationCapability) {
        // Level 1: Physiological needs
        self.update_food_water(hours_passed);
        self.update_environment(hours_passed, location);
        self.update_intimacy(hours_passed);
        self.update_rest(hours_passed);
        self.update_waste(hours_passed);
        
        // Level 2: Safety & Security (only if Level 1 is adequate)
        if self.is_need_level_active(2) {
            self.update_threat(hours_passed, location);
            self.update_income(hours_passed);
            self.update_stress(hours_passed);
            self.update_safety(hours_passed, location);
        }
        
        // Level 3: Love & Belonging (only if Level 2 is adequate)
        if self.is_need_level_active(3) {
            self.update_community(hours_passed);
        }
        
        // Level 5: Self-Actualization (only if Level 4 is adequate)
        if self.is_need_level_active(5) {
            self.update_progression(hours_passed);
        }
    }
    
    fn update_food_water(&mut self, hours_passed: u64) {
        let depletion = match &self.status {
            IndividualStatus::Working { .. } => individual_depletion::FOOD_WATER_WORKING,
            IndividualStatus::Sleeping { .. } => individual_depletion::FOOD_WATER_RESTING,
            _ => individual_depletion::FOOD_WATER_BASE,
        };
        self.food_water = (self.food_water + depletion * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    fn update_environment(&mut self, hours_passed: u64, location: &LocationCapability) {
        let depletion = if location.environmental_quality > 0.0 {
            individual_depletion::ENVIRONMENT_HEALING
        } else if location.environmental_quality < -1.0 {
            individual_depletion::ENVIRONMENT_HAZARDOUS
        } else {
            individual_depletion::ENVIRONMENT_NEUTRAL
        };
        self.environment = (self.environment + depletion * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    fn update_intimacy(&mut self, hours_passed: u64) {
        let depletion = individual_depletion::INTIMACY_BASE;
        self.intimacy = (self.intimacy + depletion * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    fn update_rest(&mut self, hours_passed: u64) {
        let depletion = match &self.status {
            IndividualStatus::Sleeping { .. } => individual_depletion::REST_SLEEPING,
            IndividualStatus::Working { .. } => individual_depletion::REST_WORKING,
            _ => individual_depletion::REST_BASE,
        };
        
        // Stress affects rest depletion
        let stress_modifier = (self.stress / 10.0) * individual_depletion::STRESS_TO_REST_FACTOR;
        
        self.rest = (self.rest + (depletion + stress_modifier) * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    fn update_waste(&mut self, hours_passed: u64) {
        let accumulation = individual_depletion::WASTE_BASE;
        self.waste = (self.waste + accumulation * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    fn update_threat(&mut self, hours_passed: u64, location: &LocationCapability) {
        let depletion = if location.provides_healthcare || location.provides_rest {
            individual_depletion::THREAT_SAFE_BUILDING
        } else if location.environmental_quality < -1.0 {
            individual_depletion::THREAT_DANGEROUS
        } else {
            individual_depletion::THREAT_BASE
        };
        self.threat = (self.threat + depletion * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    fn update_income(&mut self, hours_passed: u64) {
        let change = match &self.status {
            IndividualStatus::Working { .. } => individual_depletion::INCOME_WORKING,
            _ => individual_depletion::INCOME_LIVING_COST,
        };
        self.income = (self.income + change * hours_passed as f32)
            .clamp(0.0, thresholds::INCOME_MAX);
        
        // Low income increases security loss
        if self.income < thresholds::INCOME_CRITICAL {
            self.safety -= individual_depletion::INCOME_UNEMPLOYED * hours_passed as f32;
        }
    }
    
    fn update_stress(&mut self, hours_passed: u64) {
        let change = match &self.status {
            IndividualStatus::Working { .. } => individual_depletion::STRESS_HIGH_WORKLOAD,
            IndividualStatus::Socializing { .. } => individual_depletion::STRESS_RECREATION,
            _ => individual_depletion::STRESS_BASE,
        };
        
        // Low income adds stress
        let income_stress = if self.income < thresholds::INCOME_CRITICAL {
            individual_depletion::STRESS_LOW_INCOME
        } else {
            0.0
        };
        
        self.stress = (self.stress + (change + income_stress) * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    fn update_safety(&mut self, hours_passed: u64, location: &LocationCapability) {
        let change = if self.home_id.is_some() && location.provides_rest {
            individual_depletion::SAFETY_AT_HOME
        } else if location.provides_healthcare || location.environmental_quality > 0.0 {
            individual_depletion::SAFETY_SAFE_LOCATION
        } else if location.environmental_quality < -1.0 {
            individual_depletion::SAFETY_UNSAFE_AREA
        } else {
            individual_depletion::SAFETY_BASE
        };
        self.safety = (self.safety + change * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    fn update_community(&mut self, hours_passed: u64) {
        let depletion = match &self.status {
            IndividualStatus::Socializing { .. } => individual_depletion::COMMUNITY_EVENT,
            _ => individual_depletion::COMMUNITY_BASE,
        };
        self.community = (self.community + depletion * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX / 3.0); // Max 33.4 as per design
    }
    
    fn update_progression(&mut self, hours_passed: u64) {
        let change = match (&self.status, &self.specialized_role) {
            (IndividualStatus::Working { .. }, SpecializedRole::None) => 0.0,
            (IndividualStatus::Working { .. }, _) => individual_depletion::PROGRESSION_MEANINGFUL_WORK,
            _ => 0.0,
        };
        self.progression = (self.progression + change * hours_passed as f32)
            .clamp(0.0, thresholds::NEED_MAX);
    }
    
    /// Get the most pressing need that requires action
    pub fn get_most_pressing_need(&self) -> Option<(FundamentalNeed, f32)> {
        let mut needs = Vec::new();
        
        // Map individual needs to fundamental needs with priorities
        if self.waste > thresholds::WASTE_CRITICAL {
            needs.push((FundamentalNeed::Waste, self.waste * priority_weights::WASTE_HIGH));
        }
        
        if self.food_water < thresholds::NEED_CRITICAL_LOW {
            needs.push((FundamentalNeed::Consumption, 
                       (thresholds::NEED_MAX - self.food_water) * priority_weights::FOOD_CRITICAL));
        }
        
        if self.rest < thresholds::NEED_CRITICAL_LOW {
            needs.push((FundamentalNeed::Rest, 
                       (thresholds::NEED_MAX - self.rest) * priority_weights::REST_CRITICAL));
        }
        
        if self.environment < thresholds::NEED_CRITICAL_LOW {
            needs.push((FundamentalNeed::Environment, 
                       (thresholds::NEED_MAX - self.environment) * priority_weights::ENVIRONMENT_LOW));
        }
        
        if self.is_need_level_active(2) && self.safety < thresholds::NEED_CRITICAL_LOW {
            needs.push((FundamentalNeed::Environment, 
                       (thresholds::NEED_MAX - self.safety) * priority_weights::SAFETY_LOW));
        }
        
        if self.is_need_level_active(3) && self.community < 10.0 {
            needs.push((FundamentalNeed::Connection, 
                       (33.4 - self.community) * priority_weights::SOCIAL_NEEDS));
        }
        
        // Return the highest priority need
        needs.into_iter()
            .filter(|(_, priority)| *priority > thresholds::NEED_URGENT)
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}