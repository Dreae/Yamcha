import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { BrowserRouter } from 'react-router-dom';

import Dashboard from './Dashboard';

declare var require: any;

require('./style.scss');

ReactDOM.render(
  (
    <BrowserRouter>
      <Dashboard/>
    </BrowserRouter>
  ),
  document.getElementById("wrapper")
);