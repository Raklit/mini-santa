import * as riot from 'riot'
import { Route, Router } from '@riotjs/route';
import App from './app.riot'

import SignUpPage from './pages/signup-page.riot';
import LoginPage from './pages/login-page.riot';
import LogoutPage from './pages/logout-page.riot';
import IndexPage from './pages/index-page.riot';

import CreateInviteCodePage from './pages/create-invite-code-page.riot';
import InviteCodesPage from './pages/invite-codes-page.riot';

import PoolDetailsPage from './pages/pool-details-page.riot';
import CreatePoolPage from './pages/create-pool-page.riot';
import PoolsPage from './pages/pools-page.riot';
import AddToPoolPage from './pages/add-to-pool-page.riot';

import RoomDetailsPage from './pages/room-details-page.riot';
import RoomsPage from './pages/rooms-page.riot';


import ActionStatusInfoComponent from './action-status-info-component.riot';
import BaseLayout from './base-layout.riot';

import 'normalize.css/normalize.css'
import 'milligram/dist/milligram.min.css';

import './styles/basic.css';
import './styles/base-layout.css';

import './styles/login.css';
import './styles/signup.css';

import './styles/pools.css';
import './styles/add-to-pool.css';

import './styles/rooms.css';

riot.register('router', Router);
riot.register('route', Route);

riot.register("signup-page", SignUpPage);
riot.register("login-page", LoginPage);
riot.register("logout-page", LogoutPage)
riot.register("index-page", IndexPage);

riot.register("create-invite-code-page", CreateInviteCodePage);
riot.register("invite-codes-page", InviteCodesPage);

riot.register("create-pool-page", CreatePoolPage);
riot.register("pool-details-page", PoolDetailsPage);
riot.register("add-to-pool-page", AddToPoolPage);
riot.register("pools-page", PoolsPage);

riot.register("room-details-page", RoomDetailsPage);
riot.register("rooms-page", RoomsPage);

riot.register("action-status-info-component", ActionStatusInfoComponent);
riot.register("base-layout", BaseLayout);

const mountApp = riot.component(App)

const app = mountApp(document.getElementById('root'),{})
