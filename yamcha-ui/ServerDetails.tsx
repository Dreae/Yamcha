import { Component } from 'react';
import * as React from 'react';

type Team = 'CT' | 'T';

type ActivePlayer = {
  player_id: string,
  team: Team,
  name: string,
  kills: number,
  deaths: number,
  headshots: number,
  assists: number,
  accuracy: number,
  bot: boolean,
}

type ServerDetailState = {
  activePlayers: ActivePlayer[],
  serverId: string,
}

export default class ServerDetails extends Component<any, ServerDetailState> {
  constructor(props: any) {
    super(props);
    console.log(props);
    this.state = {
      serverId: props.match.params.serverId,
      activePlayers: [
        {
          player_id: "asd09q2",
          name: "Dreae",
          kills: 32,
          team: 'CT',
          deaths: 14,
          headshots: 27,
          assists: 4,
          accuracy: 0.58,
          bot: false,
        },
        {
          player_id: "bot1",
          name: "Norm",
          kills: 14,
          team: 'CT',
          deaths: 14,
          headshots: 1,
          assists: 4,
          accuracy: 0.38,
          bot: true,
        },
        {
          player_id: "bot2",
          name: "Brad",
          kills: 16,
          team: 'T',
          deaths: 12,
          headshots: 4,
          assists: 9,
          accuracy: 0.35,
          bot: true,
        },
        {
          player_id: "bot3",
          name: "Kyle",
          kills: 9,
          team: 'T',
          deaths: 14,
          headshots: 6,
          assists: 2,
          accuracy: 0.22,
          bot: true,
        }
      ],
    };
  }

  viewPlayer(player: ActivePlayer) {
    return () => {
      if (!player.bot) {
        this.props.history.push(`/${this.state.serverId}/player/${player.player_id}`);
      }
    };
  }

  render() {
    return (
      <div className="section">
        <h1 className="title">Current Players</h1>
        <h2 className="subtitle">The list of players currently active on the server</h2>
        <table className="table">
          <thead>
            <tr>
              <td width="5%">Team</td>
              <td>Name</td>
              <td width="5%">KDR</td>
              <td width="5%">Kills</td>
              <td width="5%">Deaths</td>
              <td width="5%">Headshots</td>
              <td width="5%">Assists</td>
              <td width="5%">Accuracy</td>
            </tr>
          </thead>
          <tbody>
            {(() => {
              if (this.state.activePlayers.length > 0) {
                return this.state.activePlayers.map((player: ActivePlayer) => {
                  return (
                    <tr key={player.player_id} onClick={this.viewPlayer(player)}>
                      <td>{player.team}</td>
                      <td>{player.name}</td>
                      <td>{(player.kills / player.deaths).toFixed(2)}</td>
                      <td>{player.kills}</td>
                      <td>{player.deaths}</td>
                      <td>{(100 * (player.headshots / player.kills)).toFixed(0) + '%'}</td>
                      <td>{player.assists}</td>
                      <td>{(100 * player.accuracy).toFixed(0) + '%'}</td>
                    </tr>
                  );
                });
              } else {
                return (
                  <div>No players currently on this server</div>
                );
              }
            })()}
          </tbody>
        </table>
      </div>
    );
  }
}