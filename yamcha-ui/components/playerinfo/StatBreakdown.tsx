import { Component } from 'react';
import * as React from 'react';
import { Link } from 'react-router-dom';

export default class StatBreakdown extends Component<any, any> {
  render() {
    return (
      <div className="card" style={{width: "100%"}}>
        <header className="card-header">
          <p className="card-header-title">
            Stats Breakdown
          </p>
          <a className="card-header-icon">
            <span className="icon">
              <i className="fa fa-bar-chart-o"></i>
            </span>
          </a>
        </header>
        <div className="card-content">
          <div className="content">
            <nav className="level">
              <div className="level-item has-text-centered">
                <div>
                  <p className="heading">Rating</p>
                  <p className="title">3,456</p>
                </div>
              </div>
              <div className="level-item has-text-centered">
                <div>
                  <p className="heading">Rank</p>
                  <p className="title">1</p>
                </div>
              </div>
            </nav>
            <hr/>
            <div className="columns">
              <div className="column">
                <strong>Kills</strong>
              </div>
              <div className="column">
                789
              </div>
              <div className="column">
                <strong>Deaths</strong>
              </div>
              <div className="column">
                456
              </div>
              <div className="column">
                <strong><abbr title="Kills per Death">KPD</abbr></strong>
              </div>
              <div className="column">
                {(789 / 456).toFixed(2)}
              </div>
            </div>
            <hr/>
            <div className="columns">
              <div className="column">
                <strong>Headshots</strong>
              </div>
              <div className="column">
                696
              </div>
              <div className="column">
                <strong><abbr title="Headshots Per Kill">HPK</abbr></strong>
              </div>
              <div className="column">
                {(696 / 789).toFixed(2)}
              </div>
              <div className="column">
                <strong>Accuracy</strong>
              </div>
              <div className="column">
                68%
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }
}