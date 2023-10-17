import time
import json
import os
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.desired_capabilities import DesiredCapabilities
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support.select import Select

class Test1():
  def setup_method(self):
    options = webdriver.FirefoxOptions()
    options.add_argument('-headless')
    self.driver = webdriver.Firefox(options=options)
    self.vars = {}

  def teardown_method(self):
    self.driver.quit()

  def wait_for_window(self, timeout = 2):
    time.sleep(round(timeout / 1000))
    wh_now = self.driver.window_handles
    wh_then = self.vars["window_handles"]
    if len(wh_now) > len(wh_then):
      return set(wh_now).difference(set(wh_then)).pop()

  def test_1(self):
    self.driver.get("https://yngal.com/")
    self.driver.find_element(By.CSS_SELECTOR, "#formSign > div:nth-child(1) > input").send_keys(os.environ.get('Y_USERNAME'))
    self.driver.find_element(By.CSS_SELECTOR, "#formSign > div:nth-child(2) > input").send_keys(os.environ.get('Y_PASSWORD'))
    self.driver.find_element(By.CSS_SELECTOR, "button:nth-child(4)").click()
    element = WebDriverWait(self.driver, 50, 0.5).until(EC.presence_of_element_located((By.CSS_SELECTOR, ".avatar")))
    print(element)
    print("开始睡眠")
    time.sleep(100)
    print("睡眠完了")

    self.driver.refresh()
    element = self.driver.find_element(By.CSS_SELECTOR,'body')
    self.vars["window_handles"] = self.driver.window_handles
    first = self.driver.find_element(By.CSS_SELECTOR, ".upDate:nth-child(1) p")
    actions = ActionChains(self.driver)
    actions.move_to_element(first).click().perform()

    self.vars["win2805"] = self.wait_for_window(2000)
    self.driver.switch_to.window(self.vars["win2805"])

    i = 0
    while i < 10:
        i += 1
        time.sleep(100)
        element = WebDriverWait(self.driver, 500, 0.5).until(EC.presence_of_element_located((By.CSS_SELECTOR, "div:nth-child(9) > .pan:nth-child(1) span")))
        if element is None:
          print("null")
          self.driver.refresh()
        else:
          print(element)
          element = self.driver.find_element(By.CSS_SELECTOR,'body')
          print('result {}'.format(element.text))

if __name__ == '__main__':
    r = Test1()
    r.setup_method()
    r.test_1()
    r.teardown_method()