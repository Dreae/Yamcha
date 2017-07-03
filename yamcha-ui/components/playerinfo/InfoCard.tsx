import { Component } from 'react';
import * as React from 'react';
import { Link } from 'react-router-dom';

export default class InfoCard extends Component<any, any> {
  render() {
    return (
      <div className="card" style={{width: "100%"}}>
        <header className="card-header">
          <p className="card-header-title">
            Player Info
          </p>
          <a className="card-header-icon">
            <span className="icon">
              <i className="fa fa-user-o"></i>
            </span>
          </a>
        </header>
        <div className="card-content">
          <div className="content">
            <div className="media">
              <div className="media-left">
                <figure className="image is-48x48">
                  <img src="http://cdn.edgecast.steamstatic.com/steamcommunity/public/images/avatars/e2/e27af2ffd8bc372e10e726c23cb19f199266443b_full.jpg" alt="Image"/>
                </figure>
              </div>
              <div className="media-content">
                <p className="title is-4">Dreae</p>
                <p className="subtitle is-6">STEAM_0:1:12204345</p>
              </div>
            </div>

            <div className="content">
              Well, here we are again.
            </div>
          </div>
          <hr/>
          <nav className="level">
            <div className="level-item has-text-centered">
              <div>
                <p className="heading">Last Joined</p>
                <p className="title">11:09 PM - 1 Jan 2016</p>
              </div>
            </div>
            <div className="level-item has-text-centered">
              <div>
                <p className="heading">Total Playtime</p>
                <p className="title">29 Hours 38 Minutes</p>
              </div>
            </div>
          </nav>
        </div>
      </div>
    );
  }
}