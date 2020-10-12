//! Amazon MWS Products API - Version 2011-10-01
//!
//! [Reference](http://docs.developer.amazonservices.com/en_US/products/Products_Overview.html)

use crate::client::{Client, Method};
use crate::result::MwsResult;

pub mod types;
pub use self::types::*;
use crate::products::types::product::Product;

static PATH: &'static str = "/Products/2011-10-01";
static VERSION: &'static str = "2011-10-01";

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct GetLowestPricedOffersForSKUResponse {
  pub Identifier: Identifier,
  pub Summary: Summary,
  pub Offers: Vec<Offer>,
}

response_envelope_type!(
  GetLowestPricedOffersForSKUResponseEnvelope<GetLowestPricedOffersForSKUResponse>,
  "GetLowestPricedOffersForSKUResponse",
  "GetLowestPricedOffersForSKUResult"
);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetLowestPricedOffersForSKUParameters {
  pub MarketplaceId: String,
  pub SellerSKU: String,
  pub ItemCondition: ItemCondition,
}

#[allow(non_snake_case)]
pub fn GetLowestPricedOffersForSKU(
  client: &Client,
  params: GetLowestPricedOffersForSKUParameters,
) -> MwsResult<GetLowestPricedOffersForSKUResponse> {
  client
    .request_xml_with_form(
      Method::Post,
      PATH,
      VERSION,
      "GetLowestPricedOffersForSKU",
      params,
    )
    .map(|e: GetLowestPricedOffersForSKUResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

response_envelope_batch_type!(
  GetMyPriceForASINResponseEnvelope<GetMyPriceForASINResult>,
  "GetMyPriceForASINResponse",
  "GetMyPriceForASINResult"
);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetMyPriceForASINParameters {
  pub MarketplaceId: String,
  #[mws_param(list_item_type_name = "ASIN")]
  pub ASINList: Vec<String>,
  pub ItemCondition: Option<ItemCondition>,
}

#[derive(FromXmlStream, Default, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct GetMyPriceForASINResult {
  #[from_xml_stream(from_attr = "ASIN")]
  pub ASIN: String,
  #[from_xml_stream(from_attr = "status")]
  pub Status: String,
  pub Product: product::Product,
}

#[allow(non_snake_case)]
pub fn GetMyPriceForASIN(
  client: &Client,
  params: GetMyPriceForASINParameters,
) -> MwsResult<Vec<GetMyPriceForASINResult>> {
  client
    .request_xml_with_form(Method::Post, PATH, VERSION, "GetMyPriceForASIN", params)
    .map(|e: GetMyPriceForASINResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

response_envelope_batch_type!(
  GetMatchingProductForIdResponseEnvelope<GetMatchingProductForIdResult>,
  "GetMatchingProductForIdResponse",
  "GetMatchingProductForIdResult"
);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetMatchingProductForIdParameters {
  pub MarketplaceId: String,
  pub IdType: String,
  #[mws_param(list_item_type_name = "Id")]
  pub IdList: Vec<String>,
}

#[derive(FromXmlStream, Default, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct GetMatchingProductForIdResult {
  #[from_xml_stream(from_attr = "Id")]
  pub Id: String,
  #[from_xml_stream(from_attr = "IdType")]
  pub IdType: String,
  #[from_xml_stream(from_attr = "status")]
  pub Status: String,
  pub Products: Vec<Product>
}

#[allow(non_snake_case)]
pub fn GetMatchingProductForId(
  client: &Client,
  params: GetMatchingProductForIdParameters,
) -> MwsResult<GetMatchingProductForIdResult> {
  client
      .request_xml_with_form(Method::Post, PATH, VERSION, "GetMatchingProductForId", params)
      .map(|e: GetMatchingProductForIdResponseEnvelope| e.into_inner())
      .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_my_price_for_sku_response() {
    test_decode_envelope!(
      GetMyPriceForASINResponseEnvelope,
      r#"
        <GetMyPriceForASINResponse xmlns="http://mws.amazonservices.com/schema/Products/2011-10-01">
          <GetMyPriceForASINResult ASIN="B073000000" status="Success">
            <Product xmlns:ns2="http://mws.amazonservices.com/schema/Products/2011-10-01/default.xsd">
              <Identifiers>
                <MarketplaceASIN>
                  <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
                  <ASIN>B073000000</ASIN>
                </MarketplaceASIN>
              </Identifiers>
              <Offers>
                <Offer>
                  <BuyingPrice>
                    <LandedPrice>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>29.99</Amount>
                    </LandedPrice>
                    <ListingPrice>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>29.99</Amount>
                    </ListingPrice>
                    <Shipping>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>0.00</Amount>
                    </Shipping>
                  </BuyingPrice>
                  <RegularPrice>
                    <CurrencyCode>USD</CurrencyCode>
                    <Amount>29.99</Amount>
                  </RegularPrice>
                  <FulfillmentChannel>AMAZON</FulfillmentChannel>
                  <ItemCondition>New</ItemCondition>
                  <ItemSubCondition>New</ItemSubCondition>
                  <SellerId>A23AS800000000</SellerId>
                  <SellerSKU>sku</SellerSKU>
                </Offer>
                <Offer>
                  <BuyingPrice>
                    <LandedPrice>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>29.99</Amount>
                    </LandedPrice>
                    <ListingPrice>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>29.99</Amount>
                    </ListingPrice>
                    <Shipping>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>0.00</Amount>
                    </Shipping>
                  </BuyingPrice>
                  <RegularPrice>
                    <CurrencyCode>USD</CurrencyCode>
                    <Amount>29.99</Amount>
                  </RegularPrice>
                  <FulfillmentChannel>MERCHANT</FulfillmentChannel>
                  <ItemCondition>New</ItemCondition>
                  <ItemSubCondition>New</ItemSubCondition>
                  <SellerId>A23AS800000000</SellerId>
                  <SellerSKU>sku-fbm</SellerSKU>
                </Offer>
              </Offers>
            </Product>
          </GetMyPriceForASINResult>
          <GetMyPriceForASINResult ASIN="B073000001" status="Success">
            <Product xmlns:ns2="http://mws.amazonservices.com/schema/Products/2011-10-01/default.xsd">
              <Identifiers>
                <MarketplaceASIN>
                  <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
                  <ASIN>B073000001</ASIN>
                </MarketplaceASIN>
              </Identifiers>
              <Offers/>
            </Product>
          </GetMyPriceForASINResult>
          <ResponseMetadata>
            <RequestId>3e353f76-2ef6-442e-a714-6bbc26f96626</RequestId>
          </ResponseMetadata>
        </GetMyPriceForASINResponse>
      "#,
      vec![
        GetMyPriceForASINResult {
          ASIN: "B073000000".to_string(),
          Status: "Success".to_string(),
          Product: product::Product {
            Identifiers: product::Identifiers {
              MarketplaceASIN: Some(product::MarketplaceASIN {
                MarketplaceId: "ATVPDKIKX0DER".to_string(),
                ASIN: "B073000000".to_string(),
              }),
              ..Default::default()
            },
            Offers: vec![
              product::Offer {
                BuyingPrice: product::Price {
                  LandedPrice: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "29.99".to_string(),
                  },
                  ListingPrice: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "29.99".to_string(),
                  },
                  Shipping: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "0.00".to_string(),
                  },
                },
                RegularPrice: MoneyType {
                  CurrencyCode: "USD".to_string(),
                  Amount: "29.99".to_string(),
                },
                FulfillmentChannel: "AMAZON".to_string(),
                ItemCondition: ItemCondition::New,
                ItemSubCondition: "New".to_string(),
                SellerId: "A23AS800000000".to_string(),
                SellerSKU: "sku".to_string(),
              },
              product::Offer {
                BuyingPrice: product::Price {
                  LandedPrice: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "29.99".to_string(),
                  },
                  ListingPrice: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "29.99".to_string(),
                  },
                  Shipping: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "0.00".to_string(),
                  },
                },
                RegularPrice: MoneyType {
                  CurrencyCode: "USD".to_string(),
                  Amount: "29.99".to_string(),
                },
                FulfillmentChannel: "MERCHANT".to_string(),
                ItemCondition: ItemCondition::New,
                ItemSubCondition: "New".to_string(),
                SellerId: "A23AS800000000".to_string(),
                SellerSKU: "sku-fbm".to_string(),
              }
            ],
            ..Default::default()
          }
        },
        GetMyPriceForASINResult {
          ASIN: "B073000001".to_string(),
          Status: "Success".to_string(),
          Product: product::Product {
            Identifiers: product::Identifiers {
              MarketplaceASIN: Some(product::MarketplaceASIN {
                MarketplaceId: "ATVPDKIKX0DER".to_string(),
                ASIN: "B073000001".to_string(),
              }),
              ..Default::default()
            },
            Offers: vec![],
            ..Default::default()
          }
        }
      ]
    );
  }

  #[test]
  fn test_get_matching_product_for_id_response() {
    test_decode_envelope!(
      GetMatchingProductForIdResponseEnvelope,
       r#"
<GetMatchingProductForIdResponse
    xmlns="http://mws.amazonservices.com/schema/Products/2011-10-01">
    <GetMatchingProductForIdResult Id="9781933988665"
        IdType="ISBN"
        status="Success">
        <Products xmlns="http://mws.amazonservices.com/schema/Products/2011-10-01"
            xmlns:ns2="http://mws.amazonservices.com/schema/Products/2011-10-01/default.xsd">
            <Product>
                <Identifiers>
                    <MarketplaceASIN>
                        <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
                        <ASIN>1933988665</ASIN>
                    </MarketplaceASIN>
                </Identifiers>
                <AttributeSets>
                    <ns2:ItemAttributes xml:lang="en-US">
                        <ns2:Author>Marmanis, Haralambos</ns2:Author>
                        <ns2:Author>Babenko, Dmitry</ns2:Author>
                        <ns2:Binding>Paperback</ns2:Binding>
                        <ns2:Edition>1</ns2:Edition>
                        <ns2:ItemDimensions>
                            <ns2:Height Units="inches">9.17</ns2:Height>
                            <ns2:Length Units="inches">7.36</ns2:Length>
                            <ns2:Width Units="inches">0.75</ns2:Width>
                            <ns2:Weight Units="pounds">1.40</ns2:Weight>
                        </ns2:ItemDimensions>
                        <ns2:IsEligibleForTradeIn>true</ns2:IsEligibleForTradeIn>
                        <ns2:Label>Manning Publications</ns2:Label>
                        <ns2:Languages>
                            <ns2:Language>
                                <ns2:Name>english</ns2:Name>
                                <ns2:Type>Unknown</ns2:Type>
                            </ns2:Language>
                            <ns2:Language>
                                <ns2:Name>english</ns2:Name>
                                <ns2:Type>Original Language</ns2:Type>
                            </ns2:Language>
                            <ns2:Language>
                                <ns2:Name>english</ns2:Name>
                                <ns2:Type>Published</ns2:Type>
                            </ns2:Language>
                        </ns2:Languages>
                        <ns2:ListPrice>
                            <ns2:Amount>44.99</ns2:Amount>
                            <ns2:CurrencyCode>USD</ns2:CurrencyCode>
                        </ns2:ListPrice>
                        <ns2:Manufacturer>Manning Publications</ns2:Manufacturer>
                        <ns2:NumberOfItems>1</ns2:NumberOfItems>
                        <ns2:NumberOfPages>368</ns2:NumberOfPages>
                        <ns2:PackageDimensions>
                            <ns2:Height Units="inches">0.80</ns2:Height>
                            <ns2:Length Units="inches">9.10</ns2:Length>
                            <ns2:Width Units="inches">7.30</ns2:Width>
                            <ns2:Weight Units="pounds">1.35</ns2:Weight>
                        </ns2:PackageDimensions>
                        <ns2:ProductGroup>Book</ns2:ProductGroup>
                        <ns2:ProductTypeName>ABIS_BOOK</ns2:ProductTypeName>
                        <ns2:PublicationDate>2009-07-05</ns2:PublicationDate>
                        <ns2:Publisher>Manning Publications</ns2:Publisher>
                        <ns2:SmallImage>
                            <ns2:URL>
                                http://ecx.images-amazon.com/images/I/51EEz05N2HL._SL75_.jpg
                            </ns2:URL>
                            <ns2:Height Units="pixels">75</ns2:Height>
                            <ns2:Width Units="pixels">60</ns2:Width>
                        </ns2:SmallImage>
                        <ns2:Studio>Manning Publications</ns2:Studio>
                        <ns2:Title>Algorithms of the Intelligent Web</ns2:Title>
                    </ns2:ItemAttributes>
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
        </Products>
    </GetMatchingProductForIdResult>
    </GetMatchingProductForIdResponse>
       "#,
       vec![
       GetMatchingProductForIdResult {
        Id: "9781933988665".to_string(),
        IdType: "ISBN".to_string(),
        Status: "Success".to_string(),
        Products: vec![
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
        ]
       }
       ]
    );
}
}
