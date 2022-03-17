npm i
npm test
npm run build

gcloud config configurations activate locationless
gsutil rsync -R build gs://app.locationless.club
