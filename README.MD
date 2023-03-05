### Environment variables:

DATABASE_URL: Postgres URL (postgres://..)

ORIGIN: URL of React server (ex. http://localhost:3000)

REDIS_URL: URL of Redis server (rediss://...)

RUST_LOG: INFO for startup log

S3_APP_ID: S3 client/appid key

S3_APP_KEY: S3 secret key

S3_BUCKET: name of S3 bucket

S3_ENDPOINT: endpoint of S3 bucket (https://s3...)

S3_REGION: region of S3 endpoint (eu-central-003)

S3_BASE_STORAGE_URL: base url of S3 objects (https://.../ with slash at the end)

**Optional:**

If desired, include these 3 environment variables to enable push notifications:

Get these values at [onesignal.com](https://onesignal.com)

ONESIGNAL_APPID: OneSignal AppId token

ONESIGNAL_APPKEY_TOKEN: OneSignal AppKey token

ONESIGNAL_USERKEY_TOKEN: OneSignal UserKey token

***

### License

Copyright © 2023 David Novakovic. Released under the AGPL-3.0 License.