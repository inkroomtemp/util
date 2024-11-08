// Generated by Selenium IDE
const { Builder, By, Key, until, Actions } = require('selenium-webdriver')

const firefox = require('selenium-webdriver/firefox');

let driver
let vars
async function waitForWindow(timeout = 2) {
    await driver.sleep(timeout)
    const handlesThen = vars["windowHandles"]
    const handlesNow = await driver.getAllWindowHandles()


    if (handlesNow.length > handlesThen.length) {
        return handlesNow.find(handle => (!handlesThen.includes(handle)))
    }
    throw new Error("New window did not appear before timeout")
}
(async function () {
    const service = new firefox.ServiceBuilder(`${process.cwd()}/geckodriver`);
    const options = new firefox.Options();
    options.addArguments('-headless');
    driver = await new Builder().forBrowser('firefox').setFirefoxOptions(options).setFirefoxService(service).build()
    vars = {}

    await driver.get("https://fufugal.com/")
    await driver.findElement(By.css("#formSign > div:nth-child(1) > input")).click()
    await driver.findElement(By.css("#formSign > div:nth-child(1) > input")).sendKeys(process.env.Y_USERNAME)
    await driver.findElement(By.css("#formSign > div:nth-child(2) > input")).click()
    await driver.findElement(By.css("#formSign > div:nth-child(2) > input")).sendKeys(process.env.Y_PASSWORD)

    const login = await driver.findElement(By.css("button:nth-child(4)"))
    await driver.actions()
        .move({ origin: login })
        .click()
        .perform()
    //    await driver.findElement(By.css("button:nth-child(4)")).click()
    await driver.sleep(5000);
    await driver.executeScript("location.reload()");
    await driver.sleep(5000);
    
    vars["windowHandles"] = await driver.getAllWindowHandles();
    const clickable = await driver.findElement(By.css(".upDate:nth-child(1) p"));// 点击第一项
    await driver.actions()
        .move({ origin: clickable })
        .click()
        .perform()


    vars["win6468"] = await waitForWindow(2000)
    await driver.switchTo().window(vars["win6468"])


    await driver.sleep(3000);
    console.log('href = ' + await driver.executeScript('return location.href'));

    var count = await driver.findElement(By.css("body"))
    console.log(await count.getText())

    await driver.get('https://fufugal.com');
    count = await driver.findElement(By.css(".el-tooltip__trigger:nth-child(1)"));
    console.log('count', await count.getText())

// 模拟滚动
    await driver.wait(until.elementIsVisible(await driver.findElement(By.css('img'))));

    let js_h = 'return document.body.clientHeight';
    let h = await driver.executeScript(js_h);
    for (let i = 1; ; i++) {
        if (i * 500 < h) {
            console.log('scroll', i, h);
            await driver.executeScript(`window.scrollTo(0,${i * 500})`);
            await driver.sleep(500);
            h = await driver.executeScript(js_h);
        } else {
            break;
        }
    }

    let height = await driver.executeScript("return Math.max(document.body.offsetHeight, document.documentElement.offsetHeight);");
    let width = await driver.executeScript("return document.body.offsetWidth;")
    console.log('width', width, height);
    await driver.manage().window().setRect({ x: 0, y: 0, width, height: height });
    await driver.sleep(3000);
    let base64 = await driver.takeScreenshot();
    const fs = require('fs');
    const path = '3.png';
    const dataBuffer = Buffer.from(base64, 'base64'); //把base64码转成buffer对象，
    fs.writeFile(path, dataBuffer, function (err) {//用fs写入文件
        if (err) {
            console.log(err);
        } else {
            console.log('写入成功！');
        }
    })

    await driver.quit();
})();
