//! https://docs.developer.amazonservices.com/en_US/products/Products_Datatypes.html

use chrono::{DateTime, Utc};
use crate::products::types::product::Product;

pub mod product {
  use super::*;

  #[allow(non_snake_case)]
  #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
  #[serde()]
  pub struct Identifiers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub MarketplaceASIN: Option<MarketplaceASIN>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub SKUIdentifier: Option<SKUIdentifier>,
  }

  #[allow(non_snake_case)]
  #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
  #[serde()]
  pub struct MarketplaceASIN {
    pub MarketplaceId: String,
    pub ASIN: String,
  }

  #[allow(non_snake_case)]
  #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
  #[serde()]
  pub struct SKUIdentifier {
    pub MarketplaceId: String,
    pub SellerId: String,
    pub SellerSKU: String,
  }

  #[allow(non_snake_case)]
  #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
  #[serde()]
  pub struct Price {
    pub LandedPrice: MoneyType,
    pub ListingPrice: MoneyType,
    pub Shipping: MoneyType,
  }

  #[allow(non_snake_case)]
  #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
  #[serde()]
  pub struct Offer {
    pub BuyingPrice: Price,
    pub RegularPrice: MoneyType,
    pub FulfillmentChannel: String,
    pub ItemCondition: ItemCondition,
    pub ItemSubCondition: String,
    pub SellerId: String,
    pub SellerSKU: String,
  }

  #[allow(non_snake_case)]
  #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
  #[serde()]
  pub struct Product {
    pub Identifiers: Identifiers,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub Offers: Vec<Offer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub AttributeSets: Vec<ItemAttributes>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub SalesRankings: Vec<SalesRank>,
  }
}

str_enum! {
  pub enum ItemCondition {
    New,
    Used,
    Collectible,
    Refurbished,
    Club,
  }
}

str_enum! {
  pub enum AvailabilityType {
    NOW,
    FUTURE_WITHOUT_DATE,
    FUTURE_WITH_DATE,
  }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct Identifier {
  pub MarketplaceId: String,
  pub SellerSKU: String,
  pub ItemCondition: ItemCondition,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub TimeOfOfferChange: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct OfferCount {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  #[from_xml_stream(from_attr = "fulfillmentChannel")]
  pub FulfillmentChannel: String,
  #[from_xml_stream(from_content)]
  pub Value: i32,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct MoneyType {
  pub Amount: String,
  pub CurrencyCode: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct Points {
  pub PointsNumber: i32,
  pub PointsMonetaryValue: MoneyType,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct LowestPrice {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  #[from_xml_stream(from_attr = "fulfillmentChannel")]
  pub FulfillmentChannel: String,
  pub LandedPrice: MoneyType,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Points: Option<Points>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct BuyBoxPrice {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  pub LandedPrice: MoneyType,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct Summary {
  pub TotalOfferCount: i32,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub NumberOfOffers: Vec<OfferCount>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub LowestPrices: Vec<LowestPrice>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub BuyBoxPrices: Vec<BuyBoxPrice>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub BuyBoxEligibleOffers: Vec<OfferCount>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct SellerFeedbackRating {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub SellerPositiveFeedbackRating: Option<String>,
  pub FeedbackCount: i32,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct ShippingTime {
  #[from_xml_stream(from_attr = "minimumHours")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub MinimumHours: Option<i32>,
  #[from_xml_stream(from_attr = "maximumHours")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub MaximumHours: Option<i32>,
  #[from_xml_stream(from_attr = "availabilityDate")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub AvailableDate: Option<DateTime<Utc>>,
  #[from_xml_stream(from_attr = "availabilityType")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub AvailabilityType: Option<AvailabilityType>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct ShipsFrom {
  pub State: String,
  pub Country: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct Offer {
  pub MyOffer: bool,
  pub SubCondition: String,
  pub SellerFeedbackRating: SellerFeedbackRating,
  pub ShippingTime: ShippingTime,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ShipsFrom: Option<ShipsFrom>,
  pub IsFulfilledByAmazon: bool,
  pub IsBuyBoxWinner: bool,
  pub IsFeaturedMerchant: bool,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct DecimalWithUnits {
  #[from_xml_stream(from_content)]
  pub Value: String,
  #[from_xml_stream(from_attr = "Units")]
  pub Units: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct NonNegativeIntegerWithUnits {
  #[from_xml_stream(from_content)]
  pub Value: i32,
  #[from_xml_stream(from_attr = "Units")]
  pub Units: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct Creator {
  #[from_xml_stream(from_content)]
  pub Name: String,
  #[from_xml_stream(from_attr = "Role")]
  pub Role: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct Dimension {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Height: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Length: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Width: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Weight: Option<DecimalWithUnits>
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct Image {
  pub URL: String,
  pub Height: DecimalWithUnits,
  pub Width: DecimalWithUnits
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct Language {
  pub Name: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Type: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub AudioFormat: Option<String>
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct SalesRank {
  pub ProductCategoryId: String,
  pub Rank: i32
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromXmlStream)]
#[serde()]
pub struct ItemAttributes {
  #[from_xml_stream(from_attr = "lang")]
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Language: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Actor: Vec<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Artist: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub AspectRatio: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub AudienceRating: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Author: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub BackFinding: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub BandMaterialType: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Binding: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub BlurayRegion: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Brand: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub CEROAgeRating: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ChainType: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ClaspType: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Color: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub CPUManufacturer: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub CPUSpeed: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub CPUType: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Creator: Vec<Creator>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Department: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Director: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub DisplaySize: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Edition: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub EpisodeSequence: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ESRBAgeRating: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Feature: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Flavor: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Format: Vec<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub GemType: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Genre: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub GolfClubFlex: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub GolfClubLoft: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub HandOrientation: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub HardDiskInterface: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub HardDiskSize: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub HardwarePlatform: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub HazardousMaterialType: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ItemDimensions: Option<Dimension>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub IsAdultProduct: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub IsAutographed: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub IsEligibleForTradeIn: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub IsMemorabilia: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub IssuesPerYear: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ItemPartNumber: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Label: Option<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Languages: Vec<Language>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub LegalDisclaimer: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ListPrice: Option<MoneyType>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Manufacturer: Option<String>,
  pub ManufacturerMaximumAge: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ManufacturerMinimumAge: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ManufacturerPartsWarrantyDescription: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub MaterialType: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub MaximumResolution: Option<DecimalWithUnits>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub MediaType: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub MetalStamp: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub MetalType: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Model: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub NumberOfDiscs: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub NumberOfIssues: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub NumberOfItems: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub NumberOfPages: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub NumberOfTracks: Option<i32>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub OperatingSystem: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub OpticalZoom: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub PackageDimensions: Option<Dimension>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub PackageQuantity: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub PartNumber: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub PegiRating: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub Platform: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ProcessorCount: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ProductGroup: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ProductTypeName: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ProductTypeSubcategory: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub PublicationDate: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Publisher: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub RegionCode: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ReleaseDate: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub RingSize: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub RunningTime: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ShaftMaterial: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Scent: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub SeasonSequence: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub SeikodoProductCode: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Size: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub SizePerPearl: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub SmallImage: Option<Image>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Studio: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub SubscriptionLength: Option<NonNegativeIntegerWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub SystemMemorySize: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub SystemMemoryType: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub TheatricalReleaseDate: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Title: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub TotalDiamondWeight: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub TotalGemWeight: Option<DecimalWithUnits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub Warranty: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub WEEETaxValue: Option<MoneyType>
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_summary() {
    test_decode!(
      Summary,
      r#"
        <TotalOfferCount>9</TotalOfferCount>
        <NumberOfOffers>
            <OfferCount condition="new" fulfillmentChannel="Amazon">3</OfferCount>
            <OfferCount condition="new" fulfillmentChannel="Merchant">6</OfferCount>
        </NumberOfOffers>
        <LowestPrices>
            <LowestPrice condition="new" fulfillmentChannel="Amazon">
                <LandedPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.99</Amount>
                </LandedPrice>
                <ListingPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.99</Amount>
                </ListingPrice>
                <Shipping>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>0.00</Amount>
                </Shipping>
            </LowestPrice>
            <LowestPrice condition="new" fulfillmentChannel="Merchant">
                <LandedPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.95</Amount>
                </LandedPrice>
                <ListingPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.95</Amount>
                </ListingPrice>
                <Shipping>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>0.00</Amount>
                </Shipping>
            </LowestPrice>
        </LowestPrices>
        <BuyBoxPrices>
            <BuyBoxPrice condition="New">
                <LandedPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.99</Amount>
                </LandedPrice>
                <ListingPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.99</Amount>
                </ListingPrice>
                <Shipping>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>0.00</Amount>
                </Shipping>
            </BuyBoxPrice>
        </BuyBoxPrices>
        <BuyBoxEligibleOffers>
            <OfferCount condition="new" fulfillmentChannel="Amazon">3</OfferCount>
            <OfferCount condition="new" fulfillmentChannel="Merchant">3</OfferCount>
        </BuyBoxEligibleOffers>
      "#,
      Summary {
        TotalOfferCount: 9,
        NumberOfOffers: vec![
          OfferCount {
            Condition: "new".to_string(),
            FulfillmentChannel: "Amazon".to_string(),
            Value: 3,
          },
          OfferCount {
            Condition: "new".to_string(),
            FulfillmentChannel: "Merchant".to_string(),
            Value: 6,
          },
        ],
        LowestPrices: vec![
          LowestPrice {
            Condition: "new".to_string(),
            FulfillmentChannel: "Amazon".to_string(),
            LandedPrice: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "239.99".to_string(),
            },
            ListingPrice: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "239.99".to_string(),
            },
            Shipping: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "0.00".to_string(),
            },
            Points: None,
          },
          LowestPrice {
            Condition: "new".to_string(),
            FulfillmentChannel: "Merchant".to_string(),
            LandedPrice: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "239.95".to_string(),
            },
            ListingPrice: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "239.95".to_string(),
            },
            Shipping: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "0.00".to_string(),
            },
            Points: None,
          }
        ],
        BuyBoxPrices: vec![BuyBoxPrice {
          Condition: "New".to_string(),
          LandedPrice: MoneyType {
            CurrencyCode: "GBP".to_string(),
            Amount: "239.99".to_string(),
          },
          ListingPrice: MoneyType {
            CurrencyCode: "GBP".to_string(),
            Amount: "239.99".to_string(),
          },
          Shipping: MoneyType {
            CurrencyCode: "GBP".to_string(),
            Amount: "0.00".to_string(),
          },
        },],
        BuyBoxEligibleOffers: vec![
          OfferCount {
            Condition: "new".to_string(),
            FulfillmentChannel: "Amazon".to_string(),
            Value: 3,
          },
          OfferCount {
            Condition: "new".to_string(),
            FulfillmentChannel: "Merchant".to_string(),
            Value: 3,
          },
        ],
      }
    );
  }

  #[test]
  fn test_offer() {
    test_decode!(
      Offer,
      r#"
        <MyOffer>false</MyOffer>
        <SubCondition>new</SubCondition>
        <SellerFeedbackRating>
            <SellerPositiveFeedbackRating>95.0</SellerPositiveFeedbackRating>
            <FeedbackCount>618</FeedbackCount>
        </SellerFeedbackRating>
        <ShippingTime minimumHours="24" maximumHours="24" availabilityType="NOW"/>
        <ListingPrice>
            <CurrencyCode>GBP</CurrencyCode>
            <Amount>239.95</Amount>
        </ListingPrice>
        <Shipping>
            <CurrencyCode>GBP</CurrencyCode>
            <Amount>0.00</Amount>
        </Shipping>
        <ShipsFrom>
            <Country>GB</Country>
        </ShipsFrom>
        <IsFulfilledByAmazon>false</IsFulfilledByAmazon>
        <IsBuyBoxWinner>false</IsBuyBoxWinner>
        <IsFeaturedMerchant>true</IsFeaturedMerchant>
      "#,
      Offer {
        MyOffer: false,
        SubCondition: "new".to_string(),
        SellerFeedbackRating: SellerFeedbackRating {
          SellerPositiveFeedbackRating: Some("95.0".to_string()),
          FeedbackCount: 618,
        },
        ShippingTime: ShippingTime {
          MinimumHours: Some(24),
          MaximumHours: Some(24),
          AvailableDate: None,
          AvailabilityType: Some(AvailabilityType::NOW),
        },
        ListingPrice: MoneyType {
          CurrencyCode: "GBP".to_string(),
          Amount: "239.95".to_string(),
        },
        Shipping: MoneyType {
          CurrencyCode: "GBP".to_string(),
          Amount: "0.00".to_string(),
        },
        ShipsFrom: Some(ShipsFrom {
          Country: "GB".to_string(),
          ..Default::default()
        }),
        IsFulfilledByAmazon: false,
        IsBuyBoxWinner: false,
        IsFeaturedMerchant: true,
      }
    );
  }
}

#[test]
fn test_product() {
  test_decode!(
    Product,
    r#"
            <Product xmlns="http://mws.amazonservices.com/schema/Products/2011-10-01" xmlns:ns2="http://mws.amazonservices.com/schema/Products/2011-10-01/default.xsd">
                <Identifiers>
                  <MarketplaceASIN>
                    <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
                    <ASIN>1933988665</ASIN>
                  </MarketplaceASIN>
                </Identifiers>
                <AttributeSets>
                    <ItemAttributes xml:lang="en-US">
                        <Author>Marmanis, Haralambos</Author>
                        <Author>Babenko, Dmitry</Author>
                        <Binding>Paperback</Binding>
                        <Edition>1</Edition>
                        <ItemDimensions>
                            <Height Units="inches">9.17</Height>
                            <Length Units="inches">7.36</Length>
                            <Width Units="inches">0.75</Width>
                            <Weight Units="pounds">1.40</Weight>
                        </ItemDimensions>
                        <IsEligibleForTradeIn>true</IsEligibleForTradeIn>
                        <Label>Manning Publications</Label>
                        <Languages>
                            <Language>
                                <Name>english</Name>
                                <Type>Unknown</Type>
                            </Language>
                            <Language>
                                <Name>english</Name>
                                <Type>Original Language</Type>
                            </Language>
                            <Language>
                                <Name>english</Name>
                                <Type>Published</Type>
                            </Language>
                        </Languages>
                        <ListPrice>
                            <Amount>44.99</Amount>
                            <CurrencyCode>USD</CurrencyCode>
                        </ListPrice>
                        <Manufacturer>Manning Publications</Manufacturer>
                        <NumberOfItems>1</NumberOfItems>
                        <NumberOfPages>368</NumberOfPages>
                        <PackageDimensions>
                            <Height Units="inches">0.80</Height>
                            <Length Units="inches">9.10</Length>
                            <Width Units="inches">7.30</Width>
                            <Weight Units="pounds">1.35</Weight>
                        </PackageDimensions>
                        <ProductGroup>Book</ProductGroup>
                        <ProductTypeName>ABIS_BOOK</ProductTypeName>
                        <PublicationDate>2009-07-05</PublicationDate>
                        <Publisher>Manning Publications</Publisher>
                        <SmallImage>
                            <URL>
                                http://ecx.images-amazon.com/images/I/51EEz05N2HL._SL75_.jpg
                            </URL>
                            <Height Units="pixels">75</Height>
                            <Width Units="pixels">60</Width>
                        </SmallImage>
                        <Studio>Manning Publications</Studio>
                        <Title>Algorithms of the Intelligent Web</Title>
                    </ItemAttributes>
                </AttributeSets>
                <Relationships/>
                <SalesRankings>
                    <SalesRank>
                        <ProductCategoryId>book_display_on_website</ProductCategoryId>
                        <Rank>59485</Rank>
                    </SalesRank>
                    <SalesRank>
                        <ProductCategoryId>377886011</ProductCategoryId>
                        <Rank>32</Rank>
                    </SalesRank>
                    <SalesRank>
                        <ProductCategoryId>3887</ProductCategoryId>
                        <Rank>66</Rank>
                    </SalesRank>
                    <SalesRank>
                        <ProductCategoryId>3870</ProductCategoryId>
                        <Rank>82</Rank>
                    </SalesRank>
                </SalesRankings>
            </Product>
    "#,
    Product {
      Identifiers: product::Identifiers {
        MarketplaceASIN: Some(product::MarketplaceASIN{
          MarketplaceId: "ATVPDKIKX0DER".to_string(),
          ASIN: "1933988665".to_string()
        }),
        ..Default::default()
      },
      AttributeSets: vec![
        ItemAttributes {
          Language: Some("en-US".to_string()),
          Author: vec![
            "Marmanis, Haralambos".to_string(),
            "Babenko, Dmitry".to_string()
          ],
          Binding: Some("Paperback".to_string()),
          Edition: Some("1".to_string()),
          ItemDimensions: Some(Dimension{
            Height: Some(DecimalWithUnits{
              Value: "9.17".to_string(),
              Units: "inches".to_string(),
            }),
            Length: Some(DecimalWithUnits{
              Value: "7.36".to_string(),
              Units: "inches".to_string()
            }),
            Width: Some(DecimalWithUnits{
              Value: "0.75".to_string(),
              Units: "inches".to_string()
            }),
            Weight: Some(DecimalWithUnits{
              Value: "1.40".to_string(),
              Units: "pounds".to_string()
            })
          }),
          IsEligibleForTradeIn: Some(true),
          Label: Some("Manning Publications".to_string()),
          Languages: vec![
            Language{
              Name: "english".to_string(),
              Type: Some("Unknown".to_string()),
              ..Default::default()
            },
            Language{
              Name: "english".to_string(),
              Type: Some("Original Language".to_string()),
              ..Default::default()
            },
            Language{
              Name: "english".to_string(),
              Type: Some("Published".to_string()),
              ..Default::default()
            }
          ],
          ListPrice: Some(MoneyType{
            Amount: "44.99".to_string(),
            CurrencyCode: "USD".to_string()
          }),
          Manufacturer: Some("Manning Publications".to_string()),
          NumberOfItems: Some(1),
          NumberOfPages: Some(368),
          PackageDimensions: Some(Dimension{
            Height: Some(DecimalWithUnits{
              Value: "0.80".to_string(),
              Units: "inches".to_string()
            }),
            Length: Some(DecimalWithUnits{
              Value: "9.10".to_string(),
              Units: "inches".to_string()
            }),
            Width: Some(DecimalWithUnits{
              Value: "7.30".to_string(),
              Units: "inches".to_string()
            }),
            Weight: Some(DecimalWithUnits{
              Value: "1.35".to_string(),
              Units: "pounds".to_string()
            })
          }),
          ProductGroup: Some("Book".to_string()),
          ProductTypeName: Some("ABIS_BOOK".to_string()),
          PublicationDate: Some("2009-07-05".to_string()),
          Publisher: Some("Manning Publications".to_string()),
          SmallImage: Some(Image{
            URL: "http://ecx.images-amazon.com/images/I/51EEz05N2HL._SL75_.jpg".to_string(),
            Height: DecimalWithUnits {
              Value: "75".to_string(),
              Units: "pixels".to_string()
            },
            Width: DecimalWithUnits {
              Value: "60".to_string(),
              Units: "pixels".to_string()
            }
          }),
          Studio: Some("Manning Publications".to_string()),
          Title: Some("Algorithms of the Intelligent Web".to_string()),
          ..Default::default()
        }
      ],
      SalesRankings: vec![
        SalesRank {
          ProductCategoryId: "book_display_on_website".to_string(),
          Rank: 59485
        },
        SalesRank {
          ProductCategoryId: "377886011".to_string(),
          Rank: 32
        },
        SalesRank {
          ProductCategoryId: "3887".to_string(),
          Rank: 66
        },
        SalesRank {
          ProductCategoryId: "3870".to_string(),
          Rank: 82
        }
      ],
      ..Default::default()
    }
  );
}