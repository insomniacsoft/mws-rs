//! https://docs.developer.amazonservices.com/en_US/products/Products_Datatypes.html

use chrono::{DateTime, Utc};
use crate::products::types::product::Product;

pub mod product {
  use super::*;

  #[allow(non_snake_case)]
  #[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
  pub struct Identifiers {
    pub MarketplaceASIN: Option<MarketplaceASIN>,
    pub SKUIdentifier: Option<SKUIdentifier>,
  }

  #[allow(non_snake_case)]
  #[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
  pub struct MarketplaceASIN {
    pub MarketplaceId: String,
    pub ASIN: String,
  }

  #[allow(non_snake_case)]
  #[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
  pub struct SKUIdentifier {
    pub MarketplaceId: String,
    pub SellerId: String,
    pub SellerSKU: String,
  }

  #[allow(non_snake_case)]
  #[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
  pub struct Price {
    pub LandedPrice: MoneyType,
    pub ListingPrice: MoneyType,
    pub Shipping: MoneyType,
  }

  #[allow(non_snake_case)]
  #[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
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
  #[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
  pub struct Product {
    pub Identifiers: Identifiers,
    pub Offers: Vec<Offer>,
    pub AttributeSets: Vec<ItemAttributes>,
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
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Identifier {
  pub MarketplaceId: String,
  pub SellerSKU: String,
  pub ItemCondition: ItemCondition,
  pub TimeOfOfferChange: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct OfferCount {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  #[from_xml_stream(from_attr = "fulfillmentChannel")]
  pub FulfillmentChannel: String,
  #[from_xml_stream(from_content)]
  pub Value: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct MoneyType {
  pub Amount: String,
  pub CurrencyCode: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Points {
  pub PointsNumber: i32,
  pub PointsMonetaryValue: MoneyType,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct LowestPrice {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  #[from_xml_stream(from_attr = "fulfillmentChannel")]
  pub FulfillmentChannel: String,
  pub LandedPrice: MoneyType,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
  pub Points: Option<Points>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct BuyBoxPrice {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  pub LandedPrice: MoneyType,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Summary {
  pub TotalOfferCount: i32,
  pub NumberOfOffers: Vec<OfferCount>,
  pub LowestPrices: Vec<LowestPrice>,
  pub BuyBoxPrices: Vec<BuyBoxPrice>,
  pub BuyBoxEligibleOffers: Vec<OfferCount>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct SellerFeedbackRating {
  pub SellerPositiveFeedbackRating: Option<String>,
  pub FeedbackCount: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ShippingTime {
  #[from_xml_stream(from_attr = "minimumHours")]
  pub MinimumHours: Option<i32>,
  #[from_xml_stream(from_attr = "maximumHours")]
  pub MaximumHours: Option<i32>,
  #[from_xml_stream(from_attr = "availabilityDate")]
  pub AvailableDate: Option<DateTime<Utc>>,
  #[from_xml_stream(from_attr = "availabilityType")]
  pub AvailabilityType: Option<AvailabilityType>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ShipsFrom {
  pub State: String,
  pub Country: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Offer {
  pub MyOffer: bool,
  pub SubCondition: String,
  pub SellerFeedbackRating: SellerFeedbackRating,
  pub ShippingTime: ShippingTime,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
  pub ShipsFrom: Option<ShipsFrom>,
  pub IsFulfilledByAmazon: bool,
  pub IsBuyBoxWinner: bool,
  pub IsFeaturedMerchant: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct DecimalWithUnits {
  #[from_xml_stream(from_content)]
  pub Value: String,
  #[from_xml_stream(from_attr = "Units")]
  pub Units: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct NonNegativeIntegerWithUnits {
  #[from_xml_stream(from_content)]
  pub Value: i32,
  #[from_xml_stream(from_attr = "Units")]
  pub Units: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Creator {
  #[from_xml_stream(from_content)]
  pub Name: String,
  #[from_xml_stream(from_attr = "Role")]
  pub Role: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Dimension {
  pub Height: Option<DecimalWithUnits>,
  pub Length: Option<DecimalWithUnits>,
  pub Width: Option<DecimalWithUnits>,
  pub Weight: Option<DecimalWithUnits>
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Image {
  pub URL: String,
  pub Height: DecimalWithUnits,
  pub Width: DecimalWithUnits
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Language {
  pub Name: String,
  pub Type: Option<String>,
  pub AudioFormat: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct SalesRank {
  pub ProductCategoryId: String,
  pub Rank: i32
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ItemAttributes {
  #[from_xml_stream(from_attr = "lang")]
  pub Language: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub Actor: Vec<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub Artist: Vec<String>,
  pub AspectRatio: Option<String>,
  pub AudienceRating: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub Author: Vec<String>,
  pub BackFinding: Option<String>,
  pub BandMaterialType: Option<String>,
  pub Binding: Option<String>,
  pub BlurayRegion: Option<String>,
  pub Brand: Option<String>,
  pub CEROAgeRating: Option<String>,
  pub ChainType: Option<String>,
  pub ClaspType: Option<String>,
  pub Color: Option<String>,
  pub CPUManufacturer: Option<String>,
  pub CPUSpeed: Option<DecimalWithUnits>,
  pub CPUType: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub Creator: Vec<Creator>,
  pub Department: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub Director: Vec<String>,
  pub DisplaySize: Option<DecimalWithUnits>,
  pub Edition: Option<String>,
  pub EpisodeSequence: Option<String>,
  pub ESRBAgeRating: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub Feature: Vec<String>,
  pub Flavor: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub Format: Vec<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub GemType: Vec<String>,
  pub Genre: Option<String>,
  pub GolfClubFlex: Option<String>,
  pub GolfClubLoft: Option<DecimalWithUnits>,
  pub HandOrientation: Option<String>,
  pub HardDiskInterface: Option<String>,
  pub HardDiskSize: Option<DecimalWithUnits>,
  pub HardwarePlatform: Option<String>,
  pub HazardousMaterialType: Option<String>,
  pub ItemDimensions: Option<Dimension>,
  pub IsAdultProduct: Option<bool>,
  pub IsAutographed: Option<bool>,
  pub IsEligibleForTradeIn: Option<bool>,
  pub IsMemorabilia: Option<bool>,
  pub IssuesPerYear: Option<String>,
  pub ItemPartNumber: Option<String>,
  pub Label: Option<String>,
  pub Languages: Vec<Language>,
  pub LegalDisclaimer: Option<String>,
  pub ListPrice: Option<MoneyType>,
  pub Manufacturer: Option<String>,
  pub ManufacturerMaximumAge: Option<DecimalWithUnits>,
  pub ManufacturerMinimumAge: Option<DecimalWithUnits>,
  pub ManufacturerPartsWarrantyDescription: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub MaterialType: Vec<String>,
  pub MaximumResolution: Option<DecimalWithUnits>,
  #[from_xml_stream(no_list_wrapper)]
  pub MediaType: Vec<String>,
  pub MetalStamp: Option<String>,
  pub MetalType: Option<String>,
  pub Model: Option<String>,
  pub NumberOfDiscs: Option<i32>,
  pub NumberOfIssues: Option<i32>,
  pub NumberOfItems: Option<i32>,
  pub NumberOfPages: Option<i32>,
  pub NumberOfTracks: Option<i32>,
  #[from_xml_stream(no_list_wrapper)]
  pub OperatingSystem: Vec<String>,
  pub OpticalZoom: Option<DecimalWithUnits>,
  pub PackageDimensions: Option<Dimension>,
  pub PackageQuantity: Option<i32>,
  pub PartNumber: Option<String>,
  pub PegiRating: Option<String>,
  #[from_xml_stream(no_list_wrapper)]
  pub Platform: Vec<String>,
  pub ProcessorCount: Option<i32>,
  pub ProductGroup: Option<String>,
  pub ProductTypeName: Option<String>,
  pub ProductTypeSubcategory: Option<String>,
  pub PublicationDate: Option<String>,
  pub Publisher: Option<String>,
  pub RegionCode: Option<String>,
  pub ReleaseDate: Option<String>,
  pub RingSize: Option<String>,
  pub RunningTime: Option<DecimalWithUnits>,
  pub ShaftMaterial: Option<String>,
  pub Scent: Option<String>,
  pub SeasonSequence: Option<String>,
  pub SeikodoProductCode: Option<String>,
  pub Size: Option<String>,
  pub SizePerPearl: Option<String>,
  pub SmallImage: Option<Image>,
  pub Studio: Option<String>,
  pub SubscriptionLength: Option<NonNegativeIntegerWithUnits>,
  pub SystemMemorySize: Option<DecimalWithUnits>,
  pub SystemMemoryType: Option<String>,
  pub TheatricalReleaseDate: Option<String>,
  pub Title: Option<String>,
  pub TotalDiamondWeight: Option<DecimalWithUnits>,
  pub TotalGemWeight: Option<DecimalWithUnits>,
  pub Warranty: Option<String>,
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