import React from 'react';

import TodoItem from './components/TodoItem';
import todoData from './data';

class App extends React.Component {
  constructor() {
    super();

    this.state = {
      todoState: todoData,
    };

    this.handleChange = this.handleChange.bind(this);
  }

  handleChange(id) {
    this.setState((prevState) => {
      let newState = JSON.parse(JSON.stringify(prevState));
      for (let i = 0; i < prevState.todoState.length; ++i) {
        if (prevState.todoState[i].id === id) {
          newState.todoState[i].completed = !newState.todoState[i].completed;
        }
      }
      return newState;
    });
  }

  render() {
    const data = this.state.todoState.map((item) => {
      return (
        <TodoItem key={item.id} item={item} handleChange={this.handleChange} />
      );
    });
    return <div className="todo-list">{data}</div>;
  }
}

export default App;
