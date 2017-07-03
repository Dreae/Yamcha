import { Component } from 'react';
import * as React from 'react';
import { Route, Link } from 'react-router-dom';

declare var require: any;

let logoUrl = require('./imgs/yamcha.png');

import ServerList from './ServerList';
import ServerDetails from './ServerDetails';
import PlayerInfo from './PlayerInfo';

export default class Dashboard extends Component {
  render() {
    return (
      <div>
        <nav className="nav has-shadow">
          <div className="nav-left">
            <Link to="/" className="nav-item">
              <img src={'/' + logoUrl} alt="Yamcha logo"/>
            </Link>
          </div>

          <div className="nav-center">
            <a className="nav-item">
              <span className="icon">
                <i className="fa fa-github"></i>
              </span>
            </a>
          </div>

          <div className="nav-right nav-menu">
            <a className="nav-item">
              Home
            </a>
            <a className="nav-item">
              Documentation
            </a>
          </div>
        </nav>
        <section className="hero is-primary has-text-centered">
          <div className="hero-body">
            <div className="container">
              <h1 className="title">
                TotallyTrash
              </h1>
              <h2 className="subtitle">
                We are totally trash.
              </h2>
            </div>
          </div>
        </section>
        <section className="section">
          <Route path="/" exact={true} component={ServerList as any}/>
          <Route path="/:serverId" exact={true} component={ServerDetails as any}/>
          <Route path="/:serverId/player/:playerId" exact={true} component={PlayerInfo as any}/>
        </section>
      </div>
    );
  }
}