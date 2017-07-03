import { Component } from 'react';
import * as React from 'react';
import { Link } from 'react-router-dom';

export default class ServerList extends Component<any, any> {
  constructor(props: any, context: any) {
    super(props, context);
    this.state = {
      serverList: [
        {
          id: "csgo2",
          name: "TotallyTrash Test Server",
          topPlayer: "Dreae",
          playerCount: 0,
          maxPlayers: 20,
        },
      ],
    };
  }

  handleClick(serverId: string) {
    return () => {
      this.props.history.push(serverId);
    }
  }

  render() {
    return (
      <table className="table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Players</th>
            <th>Top Player</th>
          </tr>
        </thead>
        <tbody>
          {this.state.serverList.map((server: any) => {
            return (
              <tr key={server.id} onClick={this.handleClick(server.id)}>
                <th>{server.name}</th>
                <th>{server.playerCount} / {server.maxPlayers}</th>
                <th>{server.topPlayer}</th>
              </tr>
            );
          })}
        </tbody>
      </table>
    );
  }
}