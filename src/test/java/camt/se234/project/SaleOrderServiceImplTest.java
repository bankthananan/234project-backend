package camt.se234.project;

import camt.se234.project.dao.OrderDao;
import camt.se234.project.entity.Product;
import camt.se234.project.entity.SaleOrder;
import camt.se234.project.entity.SaleTransaction;
import camt.se234.project.service.SaleOrderServiceImpl;

import org.junit.Before;
import org.junit.Test;

import java.util.ArrayList;
import java.util.List;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.closeTo;
import static org.hamcrest.core.Is.is;
import static org.hamcrest.Matchers.hasItems;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

public class SaleOrderServiceImplTest {
    OrderDao orderDao;
    SaleOrderServiceImpl saleOrderService;

    @Before
    public void setup(){
        orderDao = mock(OrderDao.class);
        saleOrderService = new SaleOrderServiceImpl();
        saleOrderService.setOrderDao(orderDao);
    }
    @Test
    public void getSaleOrders(){
        List<SaleTransaction> mockTransaction = new ArrayList<>();
        List<SaleOrder> mockOrder = new ArrayList<>();
        mockTransaction.add(new SaleTransaction(null,"T0001",new SaleOrder(null,"S0001",mockTransaction),
                new Product(null,"p0001","test1","test1","test1.jpg",5000),
                2));
        mockTransaction.add(new SaleTransaction(null,"T0002",new SaleOrder(null,"S0002",mockTransaction),
                new Product(null,"p0002","test2","test2","test2.jpg",2000),
                5));
        mockOrder.add(new SaleOrder(null,"S0001",mockTransaction));
        mockOrder.add(new SaleOrder(null,"S0002",mockTransaction));

        when(orderDao.getOrders()).thenReturn(mockOrder);

        assertThat(saleOrderService.getSaleOrders(),hasItems(new SaleOrder(null,"S0001",mockTransaction),
                new SaleOrder(null,"S0002",mockTransaction)));

    }
    @Test
    public void getAverageSaleOrderPrice(){
        List<SaleTransaction> mockTransaction = new ArrayList<>();
        List<SaleOrder> mockOrder = new ArrayList<>();
        mockTransaction.add(new SaleTransaction(null,"T0001",new SaleOrder(null,"S0001",mockTransaction),
                new Product(null,"p0001","test1","test1","test1.jpg",5000),
                3));
        mockTransaction.add(new SaleTransaction(null,"T0002",new SaleOrder(null,"S0002",mockTransaction),
                new Product(null,"p0002","test2","test2","test2.jpg",2000),
                2));
        mockOrder.add(new SaleOrder(null,"S0001",mockTransaction));
        mockOrder.add(new SaleOrder(null,"S0002",mockTransaction));

        when(orderDao.getOrders()).thenReturn(mockOrder);

        assertThat(saleOrderService.getAverageSaleOrderPrice(),closeTo(19000,0.1));
    }
}
