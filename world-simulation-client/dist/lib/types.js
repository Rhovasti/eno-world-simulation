/**
 * TypeScript interfaces for World Simulation data structures
 * Generated from SpacetimeDB table schemas
 */
// ===== ENUM TYPES =====
export var Valley;
(function (Valley) {
    Valley["Dawn"] = "Dawn";
    Valley["Day"] = "Day";
    Valley["Dusk"] = "Dusk";
    Valley["Night"] = "Night";
})(Valley || (Valley = {}));
export var FundamentalNeed;
(function (FundamentalNeed) {
    FundamentalNeed["Environment"] = "Environment";
    FundamentalNeed["Consumption"] = "Consumption";
    FundamentalNeed["Connection"] = "Connection";
    FundamentalNeed["Rest"] = "Rest";
    FundamentalNeed["Waste"] = "Waste";
})(FundamentalNeed || (FundamentalNeed = {}));
export var IndividualAction;
(function (IndividualAction) {
    IndividualAction["Working"] = "Working";
    IndividualAction["Socializing"] = "Socializing";
    IndividualAction["Resting"] = "Resting";
    IndividualAction["Traveling"] = "Traveling";
    IndividualAction["Consuming"] = "Consuming";
    IndividualAction["Managing"] = "Managing";
})(IndividualAction || (IndividualAction = {}));
export var SocialInteractionType;
(function (SocialInteractionType) {
    SocialInteractionType["Conversation"] = "Conversation";
    SocialInteractionType["SharedMeal"] = "SharedMeal";
    SocialInteractionType["Collaboration"] = "Collaboration";
    SocialInteractionType["Romance"] = "Romance";
    SocialInteractionType["Conflict"] = "Conflict";
    SocialInteractionType["CommunityEvent"] = "CommunityEvent";
})(SocialInteractionType || (SocialInteractionType = {}));
export var BuildingEventType;
(function (BuildingEventType) {
    BuildingEventType["Upgraded"] = "Upgraded";
    BuildingEventType["MaintenancePerformed"] = "MaintenancePerformed";
    BuildingEventType["Cleaned"] = "Cleaned";
    BuildingEventType["CapacityReached"] = "CapacityReached";
    BuildingEventType["ResourceShortage"] = "ResourceShortage";
    BuildingEventType["ProductionCompleted"] = "ProductionCompleted";
    BuildingEventType["RentCollected"] = "RentCollected";
})(BuildingEventType || (BuildingEventType = {}));
export var CityEventType;
(function (CityEventType) {
    CityEventType["Festival"] = "Festival";
    CityEventType["Election"] = "Election";
    CityEventType["Emergency"] = "Emergency";
    CityEventType["PolicyChange"] = "PolicyChange";
    CityEventType["MilestoneReached"] = "MilestoneReached";
    CityEventType["TradeAgreement"] = "TradeAgreement";
    CityEventType["InfrastructureProject"] = "InfrastructureProject";
})(CityEventType || (CityEventType = {}));
export var JobType;
(function (JobType) {
    JobType["Factory"] = "Factory";
    JobType["Office"] = "Office";
    JobType["Retail"] = "Retail";
    JobType["Healthcare"] = "Healthcare";
    JobType["Education"] = "Education";
    JobType["Research"] = "Research";
    JobType["Culture"] = "Culture";
    JobType["Utilities"] = "Utilities";
    JobType["Government"] = "Government";
})(JobType || (JobType = {}));
export var TimeOfDay;
(function (TimeOfDay) {
    TimeOfDay["Dawn"] = "dawn";
    TimeOfDay["Day"] = "day";
    TimeOfDay["Dusk"] = "dusk";
    TimeOfDay["Night"] = "night";
})(TimeOfDay || (TimeOfDay = {}));
// ===== ERROR TYPES =====
export class SimulationClientError extends Error {
    constructor(message, code, details) {
        super(message);
        this.code = code;
        this.details = details;
        this.name = 'SimulationClientError';
    }
}
//# sourceMappingURL=types.js.map