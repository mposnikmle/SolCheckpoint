import React, { FC } from 'react';
import Header from '../components/Header/Header';
import './Home.css'
import Balance from '../components/Balance/Balance';
import Collection from '../components/Collection/Collection';

interface HomeProps {
  // Define the type for your props here
}

const Home: FC<HomeProps> = (props) => {
  return (
    <>
      <Header/>
      <div className='App-main-01'>
        <div className='App-main-bal'>
          <Balance/>
        </div>
        <div className='App-main-col'>
          <Collection/>
        </div>
      </div>
    </>
  );
};

export default Home;
