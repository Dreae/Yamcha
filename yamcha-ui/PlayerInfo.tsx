import { Component } from 'react';
import * as React from 'react';
import { Link } from 'react-router-dom';

import InfoCard from './components/playerinfo/InfoCard';
import StatBreakdown from './components/playerinfo/StatBreakdown';
import PlayerWeapons from './components/playerinfo/PlayerWeapons';

export default class PlayerInfo extends Component<any, any> {
  render() {
    return (
      <div className="tile is-ancestor is-vertical">
        <div className="tile is-parent is-12">
          <div className="tile is-parent is-6 is-vertical">
            <div className="tile is-vertical">
              <InfoCard/>
            </div>
          </div>
          <div className="tile is-parent is-6">
            <div className="tile is-vertical">
              <StatBreakdown/>
            </div>
          </div>
        </div>
        <div className="tile is-parent is-12">
            <PlayerWeapons/>
        </div>
      </div>
    );
  }
}