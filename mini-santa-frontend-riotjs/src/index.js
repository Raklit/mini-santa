import * as riot from 'riot'
import { Route, Router } from '@riotjs/route';
import App from './app.riot'

import LoginPage from './pages/login-page.riot';
import IndexPage from './pages/index-page.riot';

import BaseLayout from './base-layout.riot';

import 'normalize.css/normalize.css'
import 'milligram/dist/milligram.min.css';

import './styles/base-layout.css';

import './styles/login.css';

import './styles/basic.css';

riot.register('router', Router);
riot.register('route', Route);

riot.register("login-page", LoginPage);
riot.register("index-page", IndexPage);

riot.register("base-layout", BaseLayout);

const mountApp = riot.component(App)

const app = mountApp(document.getElementById('root'),{})
