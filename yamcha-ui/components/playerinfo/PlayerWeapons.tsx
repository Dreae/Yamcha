import { Component } from 'react';
import * as React from 'react';
import { Link } from 'react-router-dom';

export default class PlayerWeapons extends Component<any, any> {
  render() {
    return (
      <div className="card" style={{width: "100%"}}>
        <header className="card-header">
          <p className="card-header-title">
            Weapon Usage
          </p>
          <a className="card-header-icon">
            <span className="icon">
              <i className="fa fa-percent"></i>
            </span>
          </a>
        </header>
        <div className="card-content">
          <div className="content">
            <table className="table">
              <thead>
                <tr>
                  <td>Gun</td>
                  <td width="5%">Position</td>
                  <td width="5%">Kills</td>
                  <td width="5%">Headshots</td>
                  <td width="5%">% HS</td>
                  <td width="5%">Accuracy</td>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td>SSG-08</td>
                  <td>1</td>
                  <td>489</td>
                  <td>475</td>
                  <td>{(100 * (475 / 489)).toFixed(0) + '%'}</td>
                  <td>89%</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    );
  }
}