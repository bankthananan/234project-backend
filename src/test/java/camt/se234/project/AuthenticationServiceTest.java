package camt.se234.project;

import camt.se234.project.dao.UserDao;
import camt.se234.project.dao.UserDaoImpl;
import camt.se234.project.entity.User;
import camt.se234.project.service.AuthenticationServiceImpl;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;

import java.util.ArrayList;
import java.util.List;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.is;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;


@RunWith(SpringRunner.class)
@SpringBootTest
public class AuthenticationServiceTest {
    UserDao userDao;
    AuthenticationServiceImpl authenticationService;

    @Before
    public void setup(){
        userDao = mock(UserDao.class);
        authenticationService = new AuthenticationServiceImpl();
        authenticationService.setUserDao(userDao);
    }

    @Test
    public void AuthenticationServiceTest(){

        when(userDao.getUser("admin","admin")).thenReturn(new User(null,"admin","admin","admin"));
        when(userDao.getUser("A","A")).thenReturn(new User(null,"A","A","user"));
        when(userDao.getUser("B","B")).thenReturn(new User(null,"B","B","user"));

        assertThat(userDao.getUser("admin","admin"),is(new User(null,"admin","admin","admin")));
        assertThat(userDao.getUser("A","A"),is(new User(null,"A","A","user")));
        assertThat(userDao.getUser("B","B"),is(new User(null,"B","B","user")));
    }
}
