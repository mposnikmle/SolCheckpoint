import React, { FC } from 'react';
import logo from '../../assets/solchkpntlogo.png';
import './Header.css';

interface HeaderProps {
  // Define the type for your props here
}

const Header: FC<HeaderProps> = (props) => {
  return (
    <>
      <header className='App-header'>
      <img className='App-logo' src={logo} alt="Logo" />
      </header>
    </>
  );
};

export default Header;