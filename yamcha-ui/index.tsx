import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { BrowserRouter } from 'react-router-dom';

import Dashboard from './Dashboard';

declare var require: any;

require('./style.scss');
require('file-loader?name=[name].[ext]!./index.html');

ReactDOM.render(
  (
    <BrowserRouter>
      <Dashboard/>
    </BrowserRouter>
  ),
  document.getElementById("wrapper")
);