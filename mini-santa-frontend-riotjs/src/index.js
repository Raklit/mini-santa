import * as riot from 'riot'
import { Route, Router } from '@riotjs/route';
import App from './app.riot'

import SignUpPage from './pages/signup-page.riot';
import LoginPage from './pages/login-page.riot';
import LogoutPage from './pages/logout-page.riot';
import IndexPage from './pages/index-page.riot';

import CreatePoolPage from './pages/create-pool-page.riot';
import PoolsPage from './pages/pools-page.riot';

import ActionStatusInfoComponent from './action-status-info-component.riot';
import BaseLayout from './base-layout.riot';

import 'normalize.css/normalize.css'
import 'milligram/dist/milligram.min.css';

import './styles/base-layout.css';

import './styles/login.css';
import './styles/pools.css';

import './styles/basic.css';

riot.register('router', Router);
riot.register('route', Route);

riot.register("signup-page", SignUpPage);
riot.register("login-page", LoginPage);
riot.register("logout-page", LogoutPage)
riot.register("index-page", IndexPage);

riot.register("create-pool-page", CreatePoolPage);
riot.register("pools-page", PoolsPage);

riot.register("action-status-info-component", ActionStatusInfoComponent);
riot.register("base-layout", BaseLayout);

const mountApp = riot.component(App)

const app = mountApp(document.getElementById('root'),{})
