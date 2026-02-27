import 'dracula-ui/styles/dracula-ui.css';
import './style.scss';

import("./pkg").then(module => {
  module.run_app();
});
