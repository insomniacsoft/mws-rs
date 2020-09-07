extern crate mws;
use chrono::offset::TimeZone;
use chrono::{DateTime, NaiveDate, Utc};
use std::path::PathBuf;
use structopt::StructOpt;

use mws::client::Client;

mod env;

use self::env::Env;

#[derive(Debug, StructOpt)]
#[structopt(name = "mws-cli", about = "MWS CLI.")]
struct Opt {
  #[structopt(long = "env", parse(from_os_str))]
  env: Option<PathBuf>,
  #[structopt(subcommand)]
  cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
  ReportRequest {
    #[structopt(long = "report_type")]
    report_type: String,
    #[structopt(long = "start_date", parse(try_from_str))]
    start_date: Option<NaiveDate>,
    #[structopt(long = "end_date", parse(try_from_str))]
    end_date: Option<NaiveDate>,
    #[structopt(long = "report_options")]
    report_options: Option<String>,
    #[structopt(long = "marketplace")]
    marketplace_id_list: Option<Vec<String>>,
  },
  ReportListRequestByIds {
    #[structopt(long = "id")]
    ids: Vec<String>,
  },
  ReportListRequestByTypes {
    #[structopt(long = "type")]
    types: Vec<String>,
  },
  ReportScheduleList {
    #[structopt(long = "type")]
    types: Vec<String>,
  },
  ReportGet {
    #[structopt(long = "id")]
    id: String,
    #[structopt(long = "out", parse(from_os_str))]
    out: PathBuf,
  },
  EncodingConvJp {
    #[structopt(long = "in", parse(from_os_str))]
    input: PathBuf,
    #[structopt(long = "out", parse(from_os_str))]
    out: PathBuf,
  },
  ProductGetLowestPricedOffersForSKU {
    #[structopt(long = "marketplace")]
    marketplace_id: String,
    #[structopt(long = "seller_sku")]
    seller_sku: String,
    #[structopt(long = "condition")]
    condition: String,
  },
  ListRegisteredDestinations {
    #[structopt(long = "marketplace")]
    marketplace_id: String,
  },
  ListSubscriptions {
    #[structopt(long = "marketplace")]
    marketplace_id: String,
  },
  DeleteSubscription {
    #[structopt(long = "marketplace")]
    marketplace_id: String,
    #[structopt(long = "url")]
    url: String,
  },
  ProductGetMyPriceForASIN {
    #[structopt(long = "marketplace")]
    marketplace_id: String,
    #[structopt(long = "asin")]
    asins: Vec<String>,
    #[structopt(long = "condition")]
    condition: Option<String>,
  },
  SubmitFeed {
    #[structopt(long = "feed_type")]
    feed_type: String,
    #[structopt(long = "contant_file", parse(from_os_str))]
    content_file: PathBuf,
    #[structopt(long = "marketplace")]
    marketplace_id_list: Option<Vec<String>>,
    #[structopt(long = "content_type")]
    content_type: String,
  },
  GetFeedSubmissionInfo {
    #[structopt(long = "submission_id")]
    submission_id: String
  },
  GetFeedSubmissionResult {
    #[structopt(long = "id")]
    id: String,
    #[structopt(long = "out", parse(from_os_str))]
    out: PathBuf,
  },
}

fn main() {
  let opt = Opt::from_args();
  let env_path = opt.env.unwrap_or_else(|| PathBuf::from(".env"));

  println!("env path: {:?}", env_path);
  ::dotenv::from_path(&env_path).ok();

  let env = Env::from_env();

  println!("seller id: {}", env.seller_id);
  println!("region id: {}", env.region_id);

  let client = get_client(&env);

  match opt.cmd {
    Command::ReportRequest {
      report_type,
      start_date,
      end_date,
      report_options,
      marketplace_id_list,
    } => {
      use mws::reports::*;
      let res = RequestReport(
        &client,
        RequestReportParameters {
          ReportType: report_type,
          StartDate: start_date.map(get_utc_datetime),
          EndDate: end_date.map(get_utc_datetime),
          ReportOptions: report_options,
          MarketplaceIdList: marketplace_id_list,
        },
      )
      .unwrap();
      println!("{:#?}", res)
    }
    Command::ReportListRequestByIds { ids } => {
      use mws::reports::*;
      let res = GetReportRequestList(
        &client,
        GetReportRequestListParameters {
          ReportRequestIdList: Some(ids),
          ..Default::default()
        },
      )
      .unwrap();

      println!("{:#?}", res)
    }
    Command::ReportListRequestByTypes { types } => {
      use mws::reports::*;
      let res = GetReportRequestList(
        &client,
        GetReportRequestListParameters {
          ReportTypeList: Some(types),
          ..Default::default()
        },
      )
      .unwrap();

      println!("{:#?}", res)
    }
    Command::ReportScheduleList { types } => {
      use mws::reports::*;
      let res = GetReportScheduleList(
        &client,
        GetReportScheduleListParameters {
          ReportTypeList: Some(types),
          ..Default::default()
        },
      )
          .unwrap();

      println!("{:#?}", res)
    }
    Command::ListSubscriptions { marketplace_id } => {
      use mws::subscriptions::*;
      let res = ListSubscriptions(
        &client,
        marketplace_id,
      )
          .unwrap();
      println!("{:#?}", res)
    }
    Command::DeleteSubscription { marketplace_id, url } => {
      use mws::subscriptions::*;
      let res = UpdateSubscription(&client, CreateSubscriptionParameters {
        MarketplaceId: marketplace_id,
        Subscription: Subscription {
          NotificationType: NotificationType::ReportProcessingFinished,
          Destination: Destination {
            DeliveryChannel: DeliveryChannel::SQS,
            AttributeList: vec![AttributeKeyValue {
              Key: AttributeKey::sqsQueueUrl,
              Value: url
            }]
          },
          IsEnabled: false
        }
      }).unwrap();
      println!("{:#?}", res)
    }
    Command::ListRegisteredDestinations { marketplace_id } => {
      use mws::subscriptions::*;
      let res = ListRegisteredDestinations(
        &client,
        marketplace_id,
      )
          .unwrap();

      println!("{:#?}", res)
    }
    Command::ReportGet { id, out } => {
      use mws::reports::*;
      let mut out = std::fs::File::create(out).unwrap();
      GetReport(&client, id, &mut out).unwrap();
    }
    Command::EncodingConvJp { input, out } => {
      use encoding_rs::*;
      let bytes = std::fs::read(input).unwrap();
      let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(&bytes);
      if had_errors {
        panic!("decode error.")
      }
      println!("encoding_used: {:?}", encoding_used);
      std::fs::write(out, cow.as_ref()).unwrap();
    }
    Command::ProductGetLowestPricedOffersForSKU {
      marketplace_id,
      seller_sku,
      condition,
    } => {
      use mws::products::*;
      let res = GetLowestPricedOffersForSKU(
        &client,
        GetLowestPricedOffersForSKUParameters {
          MarketplaceId: marketplace_id,
          SellerSKU: seller_sku,
          ItemCondition: ItemCondition::from(&condition as &str),
        },
      )
      .unwrap();
      println!("{:#?}", res)
    }
    Command::ProductGetMyPriceForASIN {
      marketplace_id,
      asins,
      condition,
    } => {
      use mws::products::*;
      let res = GetMyPriceForASIN(
        &client,
        GetMyPriceForASINParameters {
          MarketplaceId: marketplace_id,
          ASINList: asins,
          ItemCondition: condition.map(|v| ItemCondition::from(&v as &str)),
        },
      )
      .unwrap();
      println!("{:#?}", res)
    },
    Command::SubmitFeed {
      feed_type,
      content_file,
      marketplace_id_list,
      content_type
    } => {
      use mws::feeds::*;
      use std::io::Cursor;
      let content = std::fs::read(content_file).unwrap();
      let digest = md5::compute(&content);
      let b64 = base64::encode(&*digest);
      let res = SubmitFeed(
        &client,
        SubmitFeedParameters {
          FeedType: feed_type,
          MarketplaceIdList: marketplace_id_list,
          PurgeAndReplace: None,
        },
        Cursor::new(content),
        b64,
        content_type
      ).unwrap();
      println!("{:#?}", res)
    },
    Command::GetFeedSubmissionInfo {
      submission_id
    } => {
      use mws::feeds::*;
      let res = GetFeedSubmissionList(&client,
                                      GetFeedSubmissionListParameters{
                                        FeedSubmissionIdList: Some(vec![submission_id]),
                                        MaxCount: None,
                                        FeedTypeList: None,
                                        FeedProcessingStatusList: None,
                                        SubmittedFromDate: None,
                                        SubmittedToDate: None
                                      }).unwrap();
      println!("{:#?}", res);
    }
    Command::GetFeedSubmissionResult { id, out } => {
      use mws::feeds::*;
      let mut out = std::fs::File::create(out).unwrap();
      GetFeedSubmissionResult(&client, id, &mut out).unwrap();
    }
  }
}

fn get_client(env: &Env) -> Client {
  use mws::client::ClientOptions;
  use mws::constants;
  let region = constants::get_region(&env.region_id).expect("invalid region id");
  let opts = ClientOptions {
    endpoint: region.endpoint.to_string(),
    seller_id: env.seller_id.clone(),
    mws_auth_token: env.auth_token.clone(),
    aws_access_key_id: env.access_key_id.clone(),
    secret_key: env.secret_key.clone(),
  };
  Client::new(opts).unwrap()
}

fn get_utc_datetime(date: NaiveDate) -> DateTime<Utc> {
  Utc.from_utc_date(&date).and_hms(0, 0, 0)
}
