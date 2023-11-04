import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
    vus: 1000,
    duration: '30s',
};

export default function () {
    http.get('https://figure.novakovic.be/figures/landing-page');

    sleep(.2);
}